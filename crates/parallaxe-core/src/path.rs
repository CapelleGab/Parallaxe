//! Lossless conversion between native paths and bytes stored in the catalog.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[cfg(unix)]
pub fn to_bytes(path: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    path.as_os_str().as_bytes().to_vec()
}

#[cfg(unix)]
pub fn from_bytes(bytes: Vec<u8>) -> PathBuf {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(bytes).into()
}

// Windows paths are UTF-16 and may hold unpaired surrogates: store raw code units.
#[cfg(windows)]
pub fn to_bytes(path: &Path) -> Vec<u8> {
    use std::os::windows::ffi::OsStrExt;
    path.as_os_str()
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect()
}

#[cfg(windows)]
pub fn from_bytes(bytes: Vec<u8>) -> PathBuf {
    use std::os::windows::ffi::OsStringExt;
    let (pairs, _) = bytes.as_chunks::<2>();
    let wide: Vec<u16> = pairs.iter().map(|&pair| u16::from_le_bytes(pair)).collect();
    OsString::from_wide(&wide).into()
}

/// True when one root contains the other, which would index the same files twice.
pub fn overlaps(a: &Path, b: &Path) -> bool {
    a.starts_with(b) || b.starts_with(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_regular_paths() {
        let path = PathBuf::from("textures/médiéval/木材.png");
        assert_eq!(from_bytes(to_bytes(&path)), path);
    }

    #[cfg(unix)]
    #[test]
    fn round_trips_non_utf8_unix_paths() {
        use std::os::unix::ffi::OsStringExt;
        let path = PathBuf::from(OsString::from_vec(vec![b'a', 0xff, b'b']));
        assert_eq!(from_bytes(to_bytes(&path)), path);
    }

    #[cfg(windows)]
    #[test]
    fn round_trips_unpaired_surrogates_on_windows() {
        use std::os::windows::ffi::OsStringExt;
        let path = PathBuf::from(OsString::from_wide(&[0x61, 0xD800, 0x62]));
        assert_eq!(from_bytes(to_bytes(&path)), path);
    }

    #[test]
    fn detects_nested_roots() {
        let assets = Path::new("/data/assets");
        assert!(overlaps(assets, Path::new("/data/assets/textures")));
        assert!(overlaps(Path::new("/data"), assets));
        assert!(overlaps(assets, assets));
    }

    #[test]
    fn compares_whole_components() {
        assert!(!overlaps(
            Path::new("/data/assets"),
            Path::new("/data/assets-old")
        ));
        assert!(!overlaps(Path::new("/data/a"), Path::new("/data/b")));
    }
}
