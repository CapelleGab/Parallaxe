//! Walks a root and describes its files. Read-only: it never opens a file for writing.

use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use parallaxe_core::asset::AssetKind;
use walkdir::{DirEntry, WalkDir};

/// Names that are never cataloged, on top of hidden entries.
const IGNORED_NAMES: &[&str] = &[
    "Thumbs.db",
    "desktop.ini",
    "$RECYCLE.BIN",
    "System Volume Information",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedFile {
    /// Relative to the root, so a root can move without touching its assets.
    pub rel_path: PathBuf,
    pub filename: String,
    /// Lowercase, without the dot. Empty when the file has none.
    pub ext: String,
    pub kind: AssetKind,
    pub size_bytes: u64,
    pub mtime_ms: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
    /// Symbolic links are not followed: they could leave the root or loop.
    Symlink,
    /// Neither a regular file nor a directory (socket, device…).
    Special,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanItem {
    File(ScannedFile),
    Skipped {
        rel_path: PathBuf,
        reason: SkipReason,
    },
    /// This entry could not be read. The scan goes on with the others.
    Unreadable {
        rel_path: PathBuf,
    },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ScanError {
    #[error("root is not reachable")]
    RootUnavailable,
}

/// Starts a scan. Items are produced lazily: stop iterating to cancel.
pub fn scan(root: &Path) -> Result<Scan, ScanError> {
    if !root.is_dir() {
        return Err(ScanError::RootUnavailable);
    }
    let walker = WalkDir::new(root)
        .follow_links(false)
        .min_depth(1)
        .into_iter()
        .filter_entry(is_visible as fn(&DirEntry) -> bool);
    Ok(Scan {
        root: root.to_path_buf(),
        walker,
    })
}

pub struct Scan {
    root: PathBuf,
    walker: walkdir::FilterEntry<walkdir::IntoIter, fn(&DirEntry) -> bool>,
}

impl Iterator for Scan {
    type Item = ScanItem;

    fn next(&mut self) -> Option<ScanItem> {
        loop {
            let entry = match self.walker.next()? {
                Ok(entry) => entry,
                Err(error) => {
                    let rel_path = error
                        .path()
                        .map(|path| relative(&self.root, path))
                        .unwrap_or_default();
                    return Some(ScanItem::Unreadable { rel_path });
                }
            };
            let rel_path = relative(&self.root, entry.path());
            let file_type = entry.file_type();

            if file_type.is_dir() {
                continue;
            }
            if file_type.is_symlink() {
                return Some(ScanItem::Skipped {
                    rel_path,
                    reason: SkipReason::Symlink,
                });
            }
            if !file_type.is_file() {
                return Some(ScanItem::Skipped {
                    rel_path,
                    reason: SkipReason::Special,
                });
            }
            return Some(match entry.metadata() {
                Ok(metadata) => ScanItem::File(describe(rel_path, &entry, &metadata)),
                Err(_) => ScanItem::Unreadable { rel_path },
            });
        }
    }
}

fn describe(rel_path: PathBuf, entry: &DirEntry, metadata: &std::fs::Metadata) -> ScannedFile {
    let ext = entry
        .path()
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    ScannedFile {
        filename: entry.file_name().to_string_lossy().into_owned(),
        kind: AssetKind::from_extension(&ext),
        ext,
        size_bytes: metadata.len(),
        mtime_ms: metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .and_then(|elapsed| i64::try_from(elapsed.as_millis()).ok()),
        rel_path,
    }
}

fn is_visible(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    !name.starts_with('.') && !IGNORED_NAMES.contains(&name.as_ref())
}

fn relative(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn files(root: &Path) -> Vec<ScannedFile> {
        let mut found: Vec<ScannedFile> = scan(root)
            .unwrap()
            .filter_map(|item| match item {
                ScanItem::File(file) => Some(file),
                _ => None,
            })
            .collect();
        found.sort_by(|a, b| a.rel_path.cmp(&b.rel_path));
        found
    }

    #[test]
    fn lists_files_recursively_with_relative_paths() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("textures/wood")).unwrap();
        fs::write(dir.path().join("readme.txt"), "hi").unwrap();
        fs::write(dir.path().join("textures/wood/Oak.PNG"), [0_u8; 10]).unwrap();

        let found = files(dir.path());
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].rel_path, PathBuf::from("readme.txt"));
        assert_eq!(found[0].kind, AssetKind::Other);

        let oak = &found[1];
        assert_eq!(
            oak.rel_path,
            Path::new("textures").join("wood").join("Oak.PNG")
        );
        assert_eq!(oak.filename, "Oak.PNG");
        assert_eq!(oak.ext, "png");
        assert_eq!(oak.kind, AssetKind::Image);
        assert_eq!(oak.size_bytes, 10);
        assert!(oak.mtime_ms.is_some());
    }

    #[test]
    fn ignores_hidden_entries_and_os_junk() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".git/objects")).unwrap();
        fs::create_dir_all(dir.path().join(".parallaxe")).unwrap();
        fs::write(dir.path().join(".git/objects/blob"), "x").unwrap();
        fs::write(dir.path().join(".DS_Store"), "x").unwrap();
        fs::write(dir.path().join("Thumbs.db"), "x").unwrap();
        fs::write(dir.path().join("model.glb"), "x").unwrap();

        let names: Vec<String> = files(dir.path()).into_iter().map(|f| f.filename).collect();
        assert_eq!(names, ["model.glb"]);
    }

    #[test]
    fn a_hidden_root_chosen_by_the_user_is_still_scanned() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join(".assets");
        fs::create_dir(&root).unwrap();
        fs::write(root.join("rock.glb"), "x").unwrap();

        assert_eq!(files(&root).len(), 1);
    }

    #[test]
    fn handles_files_without_extension() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("LICENSE"), "x").unwrap();

        let found = files(dir.path());
        assert_eq!(found[0].ext, "");
        assert_eq!(found[0].kind, AssetKind::Other);
    }

    #[test]
    fn an_empty_root_yields_nothing() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(scan(dir.path()).unwrap().count(), 0);
    }

    #[test]
    fn refuses_a_missing_root() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            scan(&dir.path().join("unplugged")).err(),
            Some(ScanError::RootUnavailable)
        );
    }

    #[test]
    fn refuses_a_file_as_root() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.png");
        fs::write(&file, "x").unwrap();
        assert_eq!(scan(&file).err(), Some(ScanError::RootUnavailable));
    }

    #[test]
    fn stops_when_the_caller_stops() {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..50 {
            fs::write(dir.path().join(format!("{i}.png")), "x").unwrap();
        }
        assert_eq!(scan(dir.path()).unwrap().take(5).count(), 5);
    }

    #[cfg(unix)]
    #[test]
    fn skips_symlinks_instead_of_following_them() {
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("secret.txt"), "x").unwrap();
        let dir = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(outside.path(), dir.path().join("escape")).unwrap();
        std::os::unix::fs::symlink(dir.path(), dir.path().join("loop")).unwrap();

        let items: Vec<ScanItem> = scan(dir.path()).unwrap().collect();
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            ScanItem::Skipped {
                reason: SkipReason::Symlink,
                ..
            }
        )));
    }

    #[cfg(unix)]
    #[test]
    fn reports_unreadable_folders_and_keeps_going() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let locked = dir.path().join("locked");
        fs::create_dir(&locked).unwrap();
        fs::write(locked.join("hidden.png"), "x").unwrap();
        fs::write(dir.path().join("visible.png"), "x").unwrap();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).unwrap();

        let items: Vec<ScanItem> = scan(dir.path()).unwrap().collect();
        fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).unwrap();

        assert!(items.contains(&ScanItem::Unreadable {
            rel_path: PathBuf::from("locked")
        }));
        assert!(
            items
                .iter()
                .any(|item| matches!(item, ScanItem::File(f) if f.filename == "visible.png"))
        );
    }
}
