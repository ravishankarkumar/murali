//! src/resources/typst/raster.rs
//!
//! Rasterize Typst SVG output into RGBA bytes using resvg + tiny-skia.

use anyhow::Result;
use resvg::tiny_skia;
use resvg::usvg;

const TYPST_SUPERSAMPLE_FACTOR: f32 = 1.6;
const TYPST_ALPHA_GAMMA: f32 = 0.82;
const TYPST_DILATION_RADIUS: u32 = 1;
const TYPST_NORMALIZE_LOW_PERCENTILE: f32 = 0.12;
const TYPST_NORMALIZE_HIGH_PERCENTILE: f32 = 0.88;
const TYPST_NORMALIZE_SCALE_MAX: f32 = 1.85;

#[derive(Debug, Clone)]
pub struct TypstRasterized {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub normalized_height_px: f32,
}

pub fn rasterize_svg_to_rgba(svg: &str, scale: f32) -> Result<TypstRasterized> {
    let opt = usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(svg, &opt)?;

    let svg_size = tree.size();
    let render_scale = (scale * TYPST_SUPERSAMPLE_FACTOR).max(1.0);

    let width_px = ((svg_size.width() * render_scale).round() as u32).max(1);
    let height_px = ((svg_size.height() * render_scale).round() as u32).max(1);

    let mut pixmap = tiny_skia::Pixmap::new(width_px, height_px)
        .ok_or_else(|| anyhow::anyhow!("Failed to allocate pixmap {}x{}", width_px, height_px))?;

    let transform = tiny_skia::Transform::from_scale(render_scale, render_scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let (rgba, width, height, normalized_height_px) =
        normalize_typst_raster(pixmap.take(), width_px, height_px);

    Ok(TypstRasterized {
        rgba,
        width,
        height,
        normalized_height_px,
    })
}

pub fn normalized_world_height(requested_height: f32, raster: &TypstRasterized) -> f32 {
    normalized_world_height_from_metrics(
        requested_height,
        raster.height,
        raster.normalized_height_px,
    )
}

pub fn normalized_world_height_from_metrics(
    requested_height: f32,
    raster_height: u32,
    normalized_height_px: f32,
) -> f32 {
    if raster_height == 0 {
        return requested_height;
    }

    let scale = (raster_height as f32 / normalized_height_px.max(1.0))
        .clamp(1.0, TYPST_NORMALIZE_SCALE_MAX);
    requested_height * scale
}

fn normalize_typst_raster(rgba: Vec<u8>, width: u32, height: u32) -> (Vec<u8>, u32, u32, f32) {
    let (cropped, cropped_w, cropped_h) = crop_transparent_bounds(rgba, width, height);
    let masked = convert_to_alpha_mask(cropped);
    let dilated = dilate_alpha_mask(masked, cropped_w, cropped_h, TYPST_DILATION_RADIUS);
    let normalized_height_px = estimate_typographic_height(&dilated, cropped_w, cropped_h);
    (dilated, cropped_w, cropped_h, normalized_height_px)
}

fn crop_transparent_bounds(rgba: Vec<u8>, width: u32, height: u32) -> (Vec<u8>, u32, u32) {
    let mut min_x = width;
    let mut min_y = height;
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    let mut found = false;

    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4 + 3) as usize;
            if rgba[idx] > 0 {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found = true;
            }
        }
    }

    if !found {
        return (rgba, width, height);
    }

    let cropped_w = max_x - min_x + 1;
    let cropped_h = max_y - min_y + 1;
    let mut cropped = vec![0u8; (cropped_w * cropped_h * 4) as usize];

    for y in 0..cropped_h {
        for x in 0..cropped_w {
            let src_x = min_x + x;
            let src_y = min_y + y;
            let src_idx = ((src_y * width + src_x) * 4) as usize;
            let dst_idx = ((y * cropped_w + x) * 4) as usize;
            cropped[dst_idx..dst_idx + 4].copy_from_slice(&rgba[src_idx..src_idx + 4]);
        }
    }

    (cropped, cropped_w, cropped_h)
}

fn convert_to_alpha_mask(mut rgba: Vec<u8>) -> Vec<u8> {
    for px in rgba.chunks_exact_mut(4) {
        let alpha = px[3];
        if alpha == 0 {
            px[0] = 255;
            px[1] = 255;
            px[2] = 255;
            continue;
        }

        let luminance = 0.2126 * px[0] as f32 + 0.7152 * px[1] as f32 + 0.0722 * px[2] as f32;
        let coverage = ((255.0 - luminance).clamp(0.0, 255.0) / 255.0) * alpha as f32;
        let normalized = (coverage / 255.0).clamp(0.0, 1.0);
        let mask_alpha = (normalized.powf(TYPST_ALPHA_GAMMA) * 255.0)
            .round()
            .clamp(0.0, 255.0) as u8;

        px[0] = 255;
        px[1] = 255;
        px[2] = 255;
        px[3] = mask_alpha;
    }

    rgba
}

fn dilate_alpha_mask(mut rgba: Vec<u8>, width: u32, height: u32, radius: u32) -> Vec<u8> {
    if radius == 0 || width == 0 || height == 0 {
        return rgba;
    }

    let source = rgba.clone();
    let radius = radius as i32;

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut max_alpha = 0u8;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
                        continue;
                    }
                    let idx = ((ny as u32 * width + nx as u32) * 4 + 3) as usize;
                    max_alpha = max_alpha.max(source[idx]);
                }
            }

            let idx = ((y as u32 * width + x as u32) * 4) as usize;
            rgba[idx] = 255;
            rgba[idx + 1] = 255;
            rgba[idx + 2] = 255;
            rgba[idx + 3] = max_alpha;
        }
    }

    rgba
}

fn estimate_typographic_height(rgba: &[u8], width: u32, height: u32) -> f32 {
    if width == 0 || height == 0 {
        return 1.0;
    }

    let mut row_sums = vec![0.0f32; height as usize];
    for y in 0..height {
        let mut sum = 0.0f32;
        for x in 0..width {
            let idx = ((y * width + x) * 4 + 3) as usize;
            sum += rgba[idx] as f32 / 255.0;
        }
        row_sums[y as usize] = sum;
    }

    let total: f32 = row_sums.iter().sum();
    if total <= f32::EPSILON {
        return height as f32;
    }

    let low_target = total * TYPST_NORMALIZE_LOW_PERCENTILE;
    let high_target = total * TYPST_NORMALIZE_HIGH_PERCENTILE;
    let mut cumulative = 0.0f32;
    let mut low_row = 0u32;
    let mut high_row = height.saturating_sub(1);
    let mut low_found = false;

    for (idx, value) in row_sums.iter().enumerate() {
        cumulative += *value;
        if !low_found && cumulative >= low_target {
            low_row = idx as u32;
            low_found = true;
        }
        if cumulative >= high_target {
            high_row = idx as u32;
            break;
        }
    }

    let band = (high_row.saturating_sub(low_row) + 1) as f32;
    let aspect = width as f32 / height.max(1) as f32;
    let dense_rows = row_sums.iter().filter(|sum| **sum > 0.0).count() as f32;
    let multi_line_bias = (dense_rows / height as f32).clamp(0.0, 1.0);

    // Mixed text + math blocks tend to have a lot of vertical whitespace and
    // smaller dense text bands than standalone equations. Give them a slightly
    // more aggressive normalization so authored blocks don't look tiny.
    let min_band_ratio = if aspect < 4.0 || multi_line_bias > 0.18 {
        0.34
    } else if aspect < 7.0 {
        0.42
    } else {
        0.5
    };

    band.clamp(height as f32 * min_band_ratio, height as f32)
}
