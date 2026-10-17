//! Image thumbnails. Sources are untrusted: every decode is bounded in size and memory.

mod cache;

use std::io::Cursor;
use std::path::Path;

use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader, Limits};

pub use cache::ThumbnailCache;

/// Longest side of a thumbnail, in pixels. Sharp enough for 2× displays at tile size.
pub const THUMBNAIL_SIZE: u32 = 320;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub max_file_bytes: u64,
    pub max_dimension: u32,
    pub max_alloc_bytes: u64,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_file_bytes: 256 * 1024 * 1024,
            max_dimension: 16_384,
            max_alloc_bytes: 512 * 1024 * 1024,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    #[error("no thumbnail for this format")]
    Unsupported,
    #[error("image exceeds the preview budget")]
    TooLarge,
    #[error("image could not be decoded")]
    Decode,
    #[error("file could not be read: {0}")]
    Io(#[from] std::io::Error),
}

pub fn supports(ext: &str) -> bool {
    matches!(ext, "png" | "jpg" | "jpeg" | "webp" | "gif")
}

/// Decodes `source` within `budget` and returns a PNG thumbnail.
pub fn image_thumbnail(source: &Path, budget: Budget) -> Result<Vec<u8>, PreviewError> {
    if std::fs::metadata(source)?.len() > budget.max_file_bytes {
        return Err(PreviewError::TooLarge);
    }

    // trust the file's bytes, not its extension
    let mut reader = ImageReader::open(source)?.with_guessed_format()?;
    if reader.format().is_none() {
        return Err(PreviewError::Unsupported);
    }
    let mut limits = Limits::default();
    limits.max_image_width = Some(budget.max_dimension);
    limits.max_image_height = Some(budget.max_dimension);
    limits.max_alloc = Some(budget.max_alloc_bytes);
    reader.limits(limits);

    let mut decoder = reader.into_decoder().map_err(classify)?;
    let orientation = decoder.orientation().map_err(classify)?;
    let mut image = DynamicImage::from_decoder(decoder).map_err(classify)?;
    image.apply_orientation(orientation);

    if image.width() > THUMBNAIL_SIZE || image.height() > THUMBNAIL_SIZE {
        image = image.thumbnail(THUMBNAIL_SIZE, THUMBNAIL_SIZE);
    }
    let mut png = Cursor::new(Vec::new());
    image
        .write_to(&mut png, ImageFormat::Png)
        .map_err(classify)?;
    Ok(png.into_inner())
}

fn classify(error: image::ImageError) -> PreviewError {
    match error {
        image::ImageError::Limits(_) => PreviewError::TooLarge,
        image::ImageError::Unsupported(_) => PreviewError::Unsupported,
        image::ImageError::IoError(error) => PreviewError::Io(error),
        _ => PreviewError::Decode,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::path::PathBuf;

    use image::{Rgba, RgbaImage};

    use super::*;

    pub fn write_image(
        dir: &Path,
        name: &str,
        width: u32,
        height: u32,
        format: ImageFormat,
    ) -> PathBuf {
        let path = dir.join(name);
        let image = RgbaImage::from_pixel(width, height, Rgba([200, 120, 40, 255]));
        DynamicImage::ImageRgba8(image)
            .to_rgb8()
            .save_with_format(&path, format)
            .unwrap();
        path
    }

    fn dimensions(png: &[u8]) -> (u32, u32) {
        let image = image::load_from_memory_with_format(png, ImageFormat::Png).unwrap();
        (image.width(), image.height())
    }

    #[test]
    fn shrinks_while_keeping_the_aspect_ratio() {
        let dir = tempfile::tempdir().unwrap();
        let wide = write_image(dir.path(), "wide.png", 1200, 600, ImageFormat::Png);
        let tall = write_image(dir.path(), "tall.jpg", 300, 900, ImageFormat::Jpeg);

        assert_eq!(
            dimensions(&image_thumbnail(&wide, Budget::default()).unwrap()),
            (320, 160)
        );
        assert_eq!(
            dimensions(&image_thumbnail(&tall, Budget::default()).unwrap()),
            (107, 320)
        );
    }

    #[test]
    fn never_enlarges_small_images() {
        let dir = tempfile::tempdir().unwrap();
        let icon = write_image(dir.path(), "icon.png", 32, 16, ImageFormat::Png);
        assert_eq!(
            dimensions(&image_thumbnail(&icon, Budget::default()).unwrap()),
            (32, 16)
        );
    }

    #[test]
    fn reads_the_real_format_whatever_the_extension() {
        let dir = tempfile::tempdir().unwrap();
        let disguised = write_image(dir.path(), "photo.png", 400, 400, ImageFormat::Jpeg);
        assert!(image_thumbnail(&disguised, Budget::default()).is_ok());
    }

    #[test]
    fn rejects_files_that_are_not_images() {
        let dir = tempfile::tempdir().unwrap();
        let fake = dir.path().join("fake.png");
        std::fs::write(&fake, "definitely not a picture").unwrap();

        assert!(matches!(
            image_thumbnail(&fake, Budget::default()),
            Err(PreviewError::Unsupported | PreviewError::Decode)
        ));
    }

    #[test]
    fn rejects_truncated_images() {
        let dir = tempfile::tempdir().unwrap();
        let path = write_image(dir.path(), "cut.png", 400, 400, ImageFormat::Png);
        let bytes = std::fs::read(&path).unwrap();
        std::fs::write(&path, &bytes[..bytes.len() / 3]).unwrap();

        assert!(image_thumbnail(&path, Budget::default()).is_err());
    }

    #[test]
    fn refuses_images_beyond_the_budget() {
        let dir = tempfile::tempdir().unwrap();
        let big = write_image(dir.path(), "big.png", 400, 400, ImageFormat::Png);

        let small_dimension = Budget {
            max_dimension: 100,
            ..Budget::default()
        };
        assert!(matches!(
            image_thumbnail(&big, small_dimension),
            Err(PreviewError::TooLarge)
        ));

        let small_file = Budget {
            max_file_bytes: 10,
            ..Budget::default()
        };
        assert!(matches!(
            image_thumbnail(&big, small_file),
            Err(PreviewError::TooLarge)
        ));
    }

    #[test]
    fn reports_missing_files() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            image_thumbnail(&dir.path().join("gone.png"), Budget::default()),
            Err(PreviewError::Io(_))
        ));
    }

    #[test]
    fn knows_which_formats_it_supports() {
        assert!(supports("png") && supports("jpeg") && supports("webp") && supports("gif"));
        assert!(!supports("glb") && !supports("psd") && !supports(""));
    }
}
