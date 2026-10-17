use std::sync::Mutex;

use parallaxe_catalog::Catalog;
use parallaxe_core::asset::{AssetKind, AssetStatus};
use parallaxe_core::id::AssetId;
use parallaxe_preview::ThumbnailCache;
use tauri::http::{Response, StatusCode, header};

use crate::state::lock;

/// Only an asset id comes from the WebView; the file path is resolved here, inside its root.
pub fn load_thumbnail(
    catalog: &Mutex<Catalog>,
    cache: &ThumbnailCache,
    raw_id: &str,
) -> Option<Vec<u8>> {
    let id: AssetId = raw_id.parse().ok()?;
    let source = lock(catalog).asset_source(id).ok()?;
    let asset = &source.asset;
    let previewable = asset.kind == AssetKind::Image
        && asset.status == AssetStatus::Available
        && parallaxe_preview::supports(&asset.ext);
    if !previewable {
        return None;
    }
    let version = format!("{:x}-{:x}", asset.size_bytes, asset.mtime_ms.unwrap_or(0));
    cache
        .get_or_create(&id.to_string(), &version, &source.path)
        .ok()
}

pub fn response(thumbnail: Option<Vec<u8>>) -> Response<Vec<u8>> {
    let builder = Response::builder();
    let response = match thumbnail {
        // the URL carries the content version, so a cached copy can be kept forever
        Some(png) => builder
            .header(header::CONTENT_TYPE, "image/png")
            .header(header::CACHE_CONTROL, "max-age=31536000, immutable")
            .body(png),
        None => builder.status(StatusCode::NOT_FOUND).body(Vec::new()),
    };
    response.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::sync::atomic::AtomicBool;

    use image::{ImageFormat, RgbImage};
    use parallaxe_core::id::LibraryId;

    use super::*;
    use crate::scan::run_scan;

    struct Setup {
        catalog: Mutex<Catalog>,
        cache: ThumbnailCache,
        library: LibraryId,
        files: tempfile::TempDir,
        _cache_dir: tempfile::TempDir,
    }

    fn setup(write: impl Fn(&Path)) -> Setup {
        let files = tempfile::tempdir().unwrap();
        write(files.path());
        let cache_dir = tempfile::tempdir().unwrap();
        let mut catalog = Catalog::open_in_memory().unwrap();
        let (library, root) = catalog
            .create_library_with_folder("Games", &fs::canonicalize(files.path()).unwrap())
            .unwrap();
        let catalog = Mutex::new(catalog);
        run_scan(&catalog, root.id, &[], &AtomicBool::new(false), |_| {}).unwrap();
        Setup {
            catalog,
            cache: ThumbnailCache::new(cache_dir.path().join("thumbnails")),
            library: library.id,
            files,
            _cache_dir: cache_dir,
        }
    }

    fn png(dir: &Path, name: &str) {
        RgbImage::new(64, 32)
            .save_with_format(dir.join(name), ImageFormat::Png)
            .unwrap();
    }

    fn id_of(s: &Setup, filename: &str) -> String {
        lock(&s.catalog)
            .list_assets(s.library, 0, 10)
            .unwrap()
            .into_iter()
            .find(|asset| asset.filename == filename)
            .unwrap()
            .id
            .to_string()
    }

    #[test]
    fn serves_thumbnails_for_images() {
        let s = setup(|dir| png(dir, "oak.png"));
        let bytes = load_thumbnail(&s.catalog, &s.cache, &id_of(&s, "oak.png")).unwrap();
        assert!(bytes.starts_with(b"\x89PNG"));
    }

    #[test]
    fn has_nothing_for_other_kinds() {
        let s = setup(|dir| fs::write(dir.join("castle.glb"), "glTF").unwrap());
        assert!(load_thumbnail(&s.catalog, &s.cache, &id_of(&s, "castle.glb")).is_none());
    }

    #[test]
    fn has_nothing_for_missing_files() {
        let s = setup(|dir| png(dir, "oak.png"));
        let id = id_of(&s, "oak.png");
        fs::remove_file(s.files.path().join("oak.png")).unwrap();
        let root = lock(&s.catalog).list_roots(s.library).unwrap()[0].id;
        run_scan(&s.catalog, root, &[], &AtomicBool::new(false), |_| {}).unwrap();

        assert!(load_thumbnail(&s.catalog, &s.cache, &id).is_none());
    }

    #[test]
    fn rejects_anything_but_a_known_asset_id() {
        let s = setup(|dir| png(dir, "oak.png"));
        for raw in [
            "",
            "../../etc/passwd",
            "oak.png",
            &AssetId::new().to_string(),
        ] {
            assert!(load_thumbnail(&s.catalog, &s.cache, raw).is_none(), "{raw}");
        }
    }

    #[test]
    fn builds_png_and_not_found_responses() {
        let ok = response(Some(vec![1, 2, 3]));
        assert_eq!(ok.status(), StatusCode::OK);
        assert_eq!(ok.headers()[header::CONTENT_TYPE], "image/png");

        let missing = response(None);
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);
        assert!(missing.body().is_empty());
    }
}
