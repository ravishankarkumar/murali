// src/resources/text/atlas.rs

use std::collections::HashMap;

use crate::resource::text::font::{LABEL_FONT_RASTER_PX, LabelFont};

/// Information required to render a glyph quad.
#[derive(Debug, Clone)]
pub struct GlyphInfo {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub size_px: [u32; 2],
    pub bearing_px: [i32; 2],
}

/// CPU-side glyph atlas.
pub struct GlyphAtlas {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub cap_height_px: f32,
    pub glyphs: HashMap<char, GlyphInfo>,
}

impl GlyphAtlas {
    /// Build a glyph atlas for ASCII characters.
    pub fn build(font: &LabelFont) -> Self {
        // Canonical raster size (pixels), shared with layout metrics.
        let px = LABEL_FONT_RASTER_PX;

        let mut glyph_bitmaps = Vec::new();
        let mut max_row_height = 0u32;

        // Rasterize glyphs
        for ch in 32u8..=126u8 {
            let ch = ch as char;
            let (metrics, bitmap) = font.font().rasterize(ch, px);

            let h = metrics.height as u32;

            max_row_height = max_row_height.max(h);

            glyph_bitmaps.push((ch, metrics, bitmap));
        }

        // Simple row packing
        let atlas_width = 512u32;
        let mut x = 0u32;
        let mut y = 0u32;

        let mut rgba = vec![0u8; (atlas_width * atlas_width * 4) as usize];
        let mut glyphs = HashMap::new();

        for (ch, metrics, bitmap) in glyph_bitmaps {
            let w = metrics.width as u32;
            let h = metrics.height as u32;

            if x + w >= atlas_width {
                x = 0;
                y += max_row_height;
            }

            for row in 0..h {
                for col in 0..w {
                    let src = bitmap[(row * w + col) as usize];
                    let dst_x = x + col;
                    let dst_y = y + row;

                    let idx = ((dst_y * atlas_width + dst_x) * 4) as usize;
                    rgba[idx + 0] = 255;
                    rgba[idx + 1] = 255;
                    rgba[idx + 2] = 255;
                    rgba[idx + 3] = src;
                }
            }

            let uv_min = [x as f32 / atlas_width as f32, y as f32 / atlas_width as f32];
            let uv_max = [
                (x + w) as f32 / atlas_width as f32,
                (y + h) as f32 / atlas_width as f32,
            ];

            glyphs.insert(
                ch,
                GlyphInfo {
                    uv_min,
                    uv_max,
                    size_px: [w, h],
                    bearing_px: [metrics.xmin, metrics.ymin],
                },
            );

            x += w;
        }

        let cap_height_px = font.metrics().cap_height;

        Self {
            width: atlas_width,
            height: atlas_width,
            rgba,
            cap_height_px,
            glyphs,
        }
    }

    /// Lookup glyph info.
    pub fn glyph(&self, ch: char) -> Option<&GlyphInfo> {
        self.glyphs.get(&ch)
    }

    pub fn cap_height_px(&self) -> f32 {
        self.cap_height_px
    }
}
