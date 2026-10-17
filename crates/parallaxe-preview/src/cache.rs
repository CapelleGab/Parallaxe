use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::{Budget, PreviewError, image_thumbnail};

/// Thumbnails on disk, named after the asset and the version of its content.
/// A changed file gets a new name, so a stale thumbnail is never served.
pub struct ThumbnailCache {
    dir: PathBuf,
    budget: Budget,
}

impl ThumbnailCache {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            budget: Budget::default(),
        }
    }

    /// Returns the cached thumbnail, generating it first if needed.
    /// `asset_key` must be a safe file name, such as an asset id.
    pub fn get_or_create(
        &self,
        asset_key: &str,
        content_version: &str,
        source: &Path,
    ) -> Result<Vec<u8>, PreviewError> {
        let path = self.dir.join(format!("{asset_key}-{content_version}.png"));
        if let Ok(bytes) = fs::read(&path) {
            return Ok(bytes);
        }

        let png = image_thumbnail(source, self.budget)?;
        fs::create_dir_all(&self.dir)?;
        self.remove_old_versions(asset_key);
        write_atomically(&path, &png)?;
        Ok(png)
    }

    fn remove_old_versions(&self, asset_key: &str) {
        let prefix = format!("{asset_key}-");
        let Ok(entries) = fs::read_dir(&self.dir) else {
            return;
        };
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with(&prefix) {
                // best effort: a leftover file only costs disk space
                let _ = fs::remove_file(entry.path());
            }
        }
    }
}

/// Write to a temporary file, then rename: a crash never leaves a half-written thumbnail.
fn write_atomically(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let temp = path.with_extension("tmp");
    let mut file = fs::File::create(&temp)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(temp, path)
}

#[cfg(test)]
mod tests {
    use image::ImageFormat;

    use super::*;
    use crate::tests::write_image;

    #[test]
    fn generates_once_then_serves_from_disk() {
        let dir = tempfile::tempdir().unwrap();
        let source = write_image(dir.path(), "oak.png", 500, 500, ImageFormat::Png);
        let cache = ThumbnailCache::new(dir.path().join("thumbs"));

        let first = cache.get_or_create("asset-1", "v1", &source).unwrap();
        fs::remove_file(&source).unwrap();
        let second = cache.get_or_create("asset-1", "v1", &source).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn a_new_content_version_replaces_the_old_thumbnail() {
        let dir = tempfile::tempdir().unwrap();
        let source = write_image(dir.path(), "oak.png", 500, 500, ImageFormat::Png);
        let thumbs = dir.path().join("thumbs");
        let cache = ThumbnailCache::new(thumbs.clone());

        cache.get_or_create("asset-1", "v1", &source).unwrap();
        cache.get_or_create("asset-1", "v2", &source).unwrap();

        let names: Vec<String> = fs::read_dir(&thumbs)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, ["asset-1-v2.png"]);
    }

    #[test]
    fn writes_nothing_next_to_the_source() {
        let dir = tempfile::tempdir().unwrap();
        let library = dir.path().join("library");
        fs::create_dir(&library).unwrap();
        let source = write_image(&library, "oak.png", 500, 500, ImageFormat::Png);
        let cache = ThumbnailCache::new(dir.path().join("thumbs"));

        cache.get_or_create("asset-1", "v1", &source).unwrap();

        assert_eq!(fs::read_dir(&library).unwrap().count(), 1);
    }

    #[test]
    fn caches_nothing_when_decoding_fails() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("fake.png");
        fs::write(&fake, "nope").unwrap();
        let thumbs = dir.path().join("thumbs");
        let cache = ThumbnailCache::new(thumbs.clone());

        assert!(cache.get_or_create("asset-1", "v1", &fake).is_err());
        assert!(!thumbs.join("asset-1-v1.png").exists());
    }
}
