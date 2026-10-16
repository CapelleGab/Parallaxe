use serde::{Deserialize, Serialize};

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

impl AssetStatus {
    /// Annotations are never dropped automatically, whatever the status.
    pub fn is_readable(self) -> bool {
        matches!(self, Self::Available | Self::Unsupported)
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
    fn only_present_files_are_readable() {
        assert!(AssetStatus::Available.is_readable());
        assert!(AssetStatus::Unsupported.is_readable());
        assert!(!AssetStatus::Missing.is_readable());
        assert!(!AssetStatus::RootOffline.is_readable());
        assert!(!AssetStatus::Error.is_readable());
    }
}
