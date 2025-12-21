// src/latex/raster.rs
//
// SVG → RGBA rasterization.
// GPU-agnostic, pure computation.

use std::path::Path;

use crate::latex::error::LatexError;

/// Result of SVG rasterization.
#[derive(Debug)]
pub struct LatexRaster {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub fn rasterize_svg(
    path: &Path,
    px_per_world_unit: f32,
    max_texture_size: u32,
) -> Result<LatexRaster, LatexError> {
    let svg_data = std::fs::read(path)?;

    let tree = usvg::Tree::from_data(
        &svg_data,
        &usvg::Options::default(),
    )
    .map_err(|e| LatexError::DviSvgmFailed(e.to_string()))?;

    // ------------------------------------------------------------
    // SVG intrinsic size (logical units, floats)
    // ------------------------------------------------------------
    let svg_size = tree.size();
    let svg_w = svg_size.width();
    let svg_h = svg_size.height();

    // ------------------------------------------------------------
    // Desired scale from RenderConfig
    // ------------------------------------------------------------
    let desired_scale = px_per_world_unit;

    // ------------------------------------------------------------
    // Clamp scale to GPU texture limits
    // ------------------------------------------------------------
    let max_tex = max_texture_size as f32;

    let max_scale_x = max_tex / svg_w;
    let max_scale_y = max_tex / svg_h;

    let scale = desired_scale
        .min(max_scale_x)
        .min(max_scale_y);

    // ------------------------------------------------------------
    // Final raster dimensions
    // ------------------------------------------------------------
    let target_width = (svg_w * scale).round() as u32;
    let target_height = (svg_h * scale).round() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height)
        .ok_or_else(|| LatexError::DviSvgmFailed("Pixmap allocation failed".into()))?;

    // ------------------------------------------------------------
    // Render SVG → RGBA
    // ------------------------------------------------------------
    let transform = tiny_skia::Transform::from_scale(scale, scale);

    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(&tree, transform, &mut pixmap_mut);

    Ok(LatexRaster {
        width: pixmap.width(),
        height: pixmap.height(),
        rgba: pixmap.take(),
    })
}
