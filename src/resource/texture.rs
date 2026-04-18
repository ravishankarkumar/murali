use anyhow::{Context, Result};
use std::path::Path;

/// CPU-side RGBA texture data that can be uploaded onto a mesh.
#[derive(Debug, Clone)]
pub struct TextureImage {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl TextureImage {
    pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            rgba,
            width,
            height,
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let image = image::open(path)
            .with_context(|| format!("Failed to open texture image at {}", path.display()))?;
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        Ok(Self::from_rgba(rgba.into_raw(), width, height))
    }
}
