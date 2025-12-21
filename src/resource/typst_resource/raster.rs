//! src/resources/typst/raster.rs
//!
//! Rasterize an SVG string into RGBA bytes using resvg + tiny-skia (CPU).
//! Returns (rgba_bytes, width_px, height_px).
use anyhow::Result;
use resvg::tiny_skia;
use resvg::usvg;

/// Rasterize svg into 8-bit RGBA bytes (row-major, left-to-right, top-to-bottom).
pub fn rasterize_svg_to_rgba(svg: &str, scale: f32) -> Result<(Vec<u8>, u32, u32)> {
    // Parse svg into usvg tree
    let opt = usvg::Options::default();
    // Important: set the dpi scale if you want; resvg handles transforms.
    let tree = resvg::usvg::Tree::from_str(svg, &opt)?;

    // Determine svg size in pixels, apply scale
    // let svg_size = tree.root.bounds().size();
    let svg_size = tree.size().clone();

    // Use floating point width/height for debugging
    let float_width = svg_size.width() * scale;
    let float_height = svg_size.height() * scale;

    println!(
        "Debug: SVG calculated size (w x h): {} x {}",
        svg_size.width(),
        svg_size.height()
    );
    println!(
        "Debug: Raster size before max(1) (w x h): {} x {}",
        float_width, float_height
    );

    let width_px = ((svg_size.width() * scale) as u32).max(1);
    let height_px = ((svg_size.height() * scale) as u32).max(1);

    // Render to tiny-skia pixmap
    let mut pixmap = tiny_skia::Pixmap::new(width_px, height_px)
        .ok_or_else(|| anyhow::anyhow!("Failed to allocate pixmap {}x{}", width_px, height_px))?;

    // FitTo::Zoom(scale) might be appropriate, but easiest is FitTo::Original and scale via transform.
    // Use resvg::RenderOptions defaults.
    // let fit = resvg::FitTo::Original;
    let mut transform = tiny_skia::Transform::from_scale(scale as f32, scale as f32);

    resvg::render(
        &tree,
        // fit,
        transform,
        &mut pixmap.as_mut(),
    );

    // tiny-skia pixmap stores BGRA premultiplied; convert to straight RGBA.
    let data = pixmap.data();
    let mut rgba: Vec<u8> = Vec::with_capacity((width_px * height_px * 4) as usize);

    // tiny-skia pixmap byte order is BGRA (premultiplied). Convert and unpremultiply.
    for px in data.chunks_exact(4) {
        let b = px[0] as f32 / 255.0;
        let g = px[1] as f32 / 255.0;
        let r = px[2] as f32 / 255.0;
        let a = px[3] as f32 / 255.0;

        if a > 0.0 {
            let r_lin = (r / a).clamp(0.0, 1.0);
            let g_lin = (g / a).clamp(0.0, 1.0);
            let b_lin = (b / a).clamp(0.0, 1.0);
            rgba.push((r_lin * 255.0) as u8);
            rgba.push((g_lin * 255.0) as u8);
            rgba.push((b_lin * 255.0) as u8);
            rgba.push((a * 255.0) as u8);
        } else {
            rgba.push(0);
            rgba.push(0);
            rgba.push(0);
            rgba.push(0);
        }
    }

    Ok((rgba, width_px, height_px))
}
