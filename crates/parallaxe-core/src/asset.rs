use std::path::PathBuf;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::id::{AssetId, RootId};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown value {0:?}")]
pub struct UnknownValue(pub String);

/// What a scan observed about a file on disk.
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

/// A cataloged file. `rel_path` is relative to its root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    pub id: AssetId,
    pub root_id: RootId,
    pub rel_path: PathBuf,
    pub filename: String,
    pub ext: String,
    pub kind: AssetKind,
    pub size_bytes: u64,
    pub mtime_ms: Option<i64>,
    pub status: AssetStatus,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    Image,
    Audio,
    Video,
    Model3d,
    Other,
}

impl AssetKind {
    pub const ALL: [Self; 5] = [
        Self::Image,
        Self::Audio,
        Self::Video,
        Self::Model3d,
        Self::Other,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::Model3d => "model3d",
            Self::Other => "other",
        }
    }

    /// Guess from the extension only; content sniffing refines it later.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "png" | "jpg" | "jpeg" | "webp" | "gif" => Self::Image,
            "wav" | "mp3" | "flac" | "ogg" => Self::Audio,
            "mp4" | "webm" => Self::Video,
            "glb" | "gltf" | "obj" | "stl" => Self::Model3d,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetStatus {
    Available,
    Missing,
    RootOffline,
    Unsupported,
    Error,
}

impl FromStr for AssetKind {
    type Err = UnknownValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|kind| kind.as_str() == s)
            .ok_or_else(|| UnknownValue(s.to_owned()))
    }
}

impl AssetStatus {
    pub const ALL: [Self; 5] = [
        Self::Available,
        Self::Missing,
        Self::RootOffline,
        Self::Unsupported,
        Self::Error,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Missing => "missing",
            Self::RootOffline => "root_offline",
            Self::Unsupported => "unsupported",
            Self::Error => "error",
        }
    }

    /// Annotations are never dropped automatically, whatever the status.
    pub fn is_readable(self) -> bool {
        matches!(self, Self::Available | Self::Unsupported)
    }
}

impl FromStr for AssetStatus {
    type Err = UnknownValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|status| status.as_str() == s)
            .ok_or_else(|| UnknownValue(s.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_mvp_formats() {
        assert_eq!(AssetKind::from_extension("PNG"), AssetKind::Image);
        assert_eq!(AssetKind::from_extension("flac"), AssetKind::Audio);
        assert_eq!(AssetKind::from_extension("webm"), AssetKind::Video);
        assert_eq!(AssetKind::from_extension("glb"), AssetKind::Model3d);
        assert_eq!(AssetKind::from_extension("Stl"), AssetKind::Model3d);
    }

    #[test]
    fn unknown_extensions_are_other() {
        assert_eq!(AssetKind::from_extension("fbx"), AssetKind::Other);
        assert_eq!(AssetKind::from_extension(""), AssetKind::Other);
    }

    #[test]
    fn serializes_as_snake_case() {
        let json = serde_json::to_string(&AssetStatus::RootOffline).unwrap();
        assert_eq!(json, "\"root_offline\"");
        let json = serde_json::to_string(&AssetKind::Model3d).unwrap();
        assert_eq!(json, "\"model3d\"");
    }

    #[test]
    fn text_form_matches_serde_and_round_trips() {
        for kind in AssetKind::ALL {
            let json = serde_json::to_value(kind).unwrap();
            assert_eq!(json, kind.as_str());
            assert_eq!(kind.as_str().parse::<AssetKind>().unwrap(), kind);
        }
        for status in AssetStatus::ALL {
            let json = serde_json::to_value(status).unwrap();
            assert_eq!(json, status.as_str());
            assert_eq!(status.as_str().parse::<AssetStatus>().unwrap(), status);
        }
    }

    #[test]
    fn rejects_unknown_text() {
        assert_eq!(
            "deleted".parse::<AssetStatus>(),
            Err(UnknownValue("deleted".into()))
        );
        assert!("mesh".parse::<AssetKind>().is_err());
    }

    #[test]
    fn only_present_files_are_readable() {
        assert!(AssetStatus::Available.is_readable());
        assert!(AssetStatus::Unsupported.is_readable());
        assert!(!AssetStatus::Missing.is_readable());
        assert!(!AssetStatus::RootOffline.is_readable());
        assert!(!AssetStatus::Error.is_readable());
    }
}
