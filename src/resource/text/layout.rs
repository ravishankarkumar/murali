// src/resources/text/layout.rs

use crate::resource::text::font::{FontMetrics, LabelFont};

/// A positioned glyph in a single-line label.
#[derive(Debug, Clone)]
pub struct GlyphInstance {
    pub ch: char,

    /// X position relative to label origin (baseline)
    pub x: f32,

    /// Advance width (before scaling)
    pub advance: f32,
}

/// Result of laying out a label.
#[derive(Debug, Clone)]
pub struct LabelLayout {
    pub glyphs: Vec<GlyphInstance>,

    /// Total width in world units
    pub width: f32,

    /// Total height in world units
    pub height: f32,
}

/// Compute glyph layout for a single-line label.
///
/// This is CPU-only and renderer-agnostic.
pub fn layout_label(
    font: &LabelFont,
    text: &str,
    world_height: f32,
) -> LabelLayout {
    let metrics: FontMetrics = font.metrics();

    // Scale font units → world units
    let scale = world_height / metrics.cap_height;

    let mut glyphs = Vec::new();
    let mut cursor_x = 0.0;

    for ch in text.chars() {
        // Query glyph metrics at canonical size
        let m = font.font().metrics(ch, 100.0);

        let advance = m.advance_width;

        glyphs.push(GlyphInstance {
            ch,
            x: cursor_x * scale,
            advance: advance * scale,
        });

        cursor_x += advance;
    }

    let width = cursor_x * scale;
    let height = world_height;

    LabelLayout {
        glyphs,
        width,
        height,
    }
}
