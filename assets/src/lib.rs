use image::DynamicImage;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum AssetError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image Error: {0}")]
    Image(#[from] image::ImageError),
    #[error("WebP Encoding Error")]
    WebPEncode,
}

#[derive(Clone, Debug)]
pub struct AssetOptimizer {
    base_dir: PathBuf,
}

impl AssetOptimizer {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Recursively processes all images in the base directory
    pub async fn process_all(&self) -> Result<(), AssetError> {
        let base_dir = self.base_dir.clone();
        
        // Spawn blocking task for CPU bound image processing
        tokio::task::spawn_blocking(move || {
            let walk_dir = WalkDir::new(&base_dir).into_iter();
            for entry in walk_dir.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        let ext = ext.to_lowercase();
                        if ext == "png" || ext == "jpg" || ext == "jpeg" {
                            // Avoid re-processing if the WebP versions already exist
                            // Actually, just process them if we don't track state
                            let _ = Self::process_image(path);
                        }
                    }
                }
            }
        })
        .await
        .unwrap();

        Ok(())
    }

    fn process_image(path: &Path) -> Result<(), AssetError> {
        let img = image::open(path)?;
        
        let parent = path.parent().unwrap_or(Path::new(""));
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");

        // Generate Original as WebP
        Self::save_webp(&img, &parent.join(format!("{}.webp", stem)))?;

        // Generate Thumbnail (256w)
        let thumb = img.resize(256, 256, image::imageops::FilterType::Lanczos3);
        Self::save_webp(&thumb, &parent.join(format!("{}-thumb.webp", stem)))?;

        // Generate Mobile (768w)
        let mobile = img.resize(768, 768, image::imageops::FilterType::Lanczos3);
        Self::save_webp(&mobile, &parent.join(format!("{}-mobile.webp", stem)))?;

        // Generate Desktop (1920w)
        let desktop = img.resize(1920, 1920, image::imageops::FilterType::Lanczos3);
        Self::save_webp(&desktop, &parent.join(format!("{}-desktop.webp", stem)))?;

        Ok(())
    }

    fn save_webp(img: &DynamicImage, output_path: &Path) -> Result<(), AssetError> {
        if output_path.exists() {
            return Ok(()); // Skip if already generated
        }
        
        let encoder = webp::Encoder::from_image(img)
            .map_err(|_| AssetError::WebPEncode)?;
            
        let webp_memory = encoder.encode(85.0); // 85% quality
        std::fs::write(output_path, &*webp_memory)?;
        
        Ok(())
    }
}
