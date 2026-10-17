//! Shapes sent to the WebView. Internal types never cross the IPC boundary directly.

use parallaxe_core::library::{Library, Root, RootStatus};
use serde::Serialize;
use ts_rs::TS;

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
            last_seen_at: root.last_seen_at,
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
            last_seen_at: Some(42),
            created_at: 1,
        };
        let json = serde_json::to_value(RootDto::from(root)).unwrap();

        assert_eq!(json["path"], "/data/assets");
        assert_eq!(json["status"], "offline");
        assert_eq!(json["lastSeenAt"], 42);
        assert!(json.get("libraryId").is_some());
    }
}
