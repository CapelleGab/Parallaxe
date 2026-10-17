//! Shapes sent to the WebView. Internal types never cross the IPC boundary directly.

use parallaxe_core::asset::{Asset, AssetKind, AssetStatus};
use parallaxe_core::library::{Library, Root, RootStatus};
use serde::Serialize;
use ts_rs::TS;

use crate::scan::{ScanCounts, ScanEnd};

#[derive(Debug, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct LibraryDto {
    pub id: String,
    pub name: String,
    pub created_at: i64,
}

impl From<Library> for LibraryDto {
    fn from(library: Library) -> Self {
        Self {
            id: library.id.to_string(),
            name: library.name,
            created_at: library.created_at,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "snake_case")]
pub enum RootStatusDto {
    Available,
    Offline,
}

#[derive(Debug, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct RootDto {
    pub id: String,
    pub library_id: String,
    /// For display only; the UI always refers to a root by its id.
    pub path: String,
    pub status: RootStatusDto,
    pub is_primary: bool,
    pub last_seen_at: Option<i64>,
}

impl From<Root> for RootDto {
    fn from(root: Root) -> Self {
        Self {
            id: root.id.to_string(),
            library_id: root.library_id.to_string(),
            path: root.path.to_string_lossy().into_owned(),
            status: match root.status {
                RootStatus::Available => RootStatusDto::Available,
                RootStatus::Offline => RootStatusDto::Offline,
            },
            is_primary: root.is_primary,
            last_seen_at: root.last_seen_at,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    pub id: String,
    pub root_id: String,
    /// Relative to its root, for display.
    pub rel_path: String,
    pub filename: String,
    pub ext: String,
    #[ts(type = "\"image\" | \"audio\" | \"video\" | \"model3d\" | \"other\"")]
    pub kind: AssetKind,
    pub size_bytes: u64,
    pub mtime_ms: Option<i64>,
    #[ts(type = "\"available\" | \"missing\" | \"root_offline\" | \"unsupported\" | \"error\"")]
    pub status: AssetStatus,
    pub is_favorite: bool,
}

impl From<Asset> for AssetDto {
    fn from(asset: Asset) -> Self {
        Self {
            id: asset.id.to_string(),
            root_id: asset.root_id.to_string(),
            rel_path: asset.rel_path.to_string_lossy().into_owned(),
            filename: asset.filename,
            ext: asset.ext,
            kind: asset.kind,
            size_bytes: asset.size_bytes,
            mtime_ms: asset.mtime_ms,
            status: asset.status,
            is_favorite: asset.is_favorite,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AssetPageDto {
    pub total: usize,
    pub items: Vec<AssetDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgressDto {
    pub files: u64,
    pub skipped: u64,
    pub unreadable: u64,
}

impl From<ScanCounts> for ScanProgressDto {
    fn from(counts: ScanCounts) -> Self {
        Self {
            files: counts.files,
            skipped: counts.skipped,
            unreadable: counts.unreadable,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase", tag = "outcome")]
pub enum ScanResultDto {
    Completed {
        progress: ScanProgressDto,
        missing: usize,
    },
    Cancelled {
        progress: ScanProgressDto,
    },
    Offline,
}

impl From<ScanEnd> for ScanResultDto {
    fn from(end: ScanEnd) -> Self {
        match end {
            ScanEnd::Completed { counts, missing } => Self::Completed {
                progress: counts.into(),
                missing,
            },
            ScanEnd::Cancelled { counts } => Self::Cancelled {
                progress: counts.into(),
            },
            ScanEnd::Offline => Self::Offline,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use parallaxe_core::id::{LibraryId, RootId};

    use super::*;

    #[test]
    fn serializes_roots_in_camel_case() {
        let root = Root {
            id: RootId::new(),
            library_id: LibraryId::new(),
            path: PathBuf::from("/data/assets"),
            status: RootStatus::Offline,
            is_primary: true,
            last_seen_at: Some(42),
            created_at: 1,
        };
        let json = serde_json::to_value(RootDto::from(root)).unwrap();

        assert_eq!(json["path"], "/data/assets");
        assert_eq!(json["status"], "offline");
        assert_eq!(json["lastSeenAt"], 42);
        assert_eq!(json["isPrimary"], true);
        assert!(json.get("libraryId").is_some());
    }

    #[test]
    fn serializes_scan_results_with_an_outcome_tag() {
        let done = ScanResultDto::from(ScanEnd::Completed {
            counts: ScanCounts {
                files: 3,
                skipped: 1,
                unreadable: 0,
            },
            missing: 2,
        });
        assert_eq!(
            serde_json::to_value(done).unwrap(),
            serde_json::json!({
                "outcome": "completed",
                "progress": { "files": 3, "skipped": 1, "unreadable": 0 },
                "missing": 2
            })
        );
        assert_eq!(
            serde_json::to_value(ScanResultDto::from(ScanEnd::Offline)).unwrap(),
            serde_json::json!({ "outcome": "offline" })
        );
    }
}
