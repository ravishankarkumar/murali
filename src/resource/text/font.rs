// src/resources/text/font.rs

use fontdue::Font;

pub const LABEL_FONT_RASTER_PX: f32 = 64.0;

/// Font metrics normalized to font units.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    /// Canonical raster size used for both atlas generation and layout metrics.
    pub raster_px: f32,

    /// Distance from baseline to top of capital letters
    pub cap_height: f32,

    /// Full ascent (baseline → highest point)
    pub ascent: f32,

    /// Full descent (baseline → lowest point, positive value)
    pub descent: f32,

    /// Line height recommended by the font
    pub line_height: f32,
}

/// Loaded font + cached metrics.
///
/// Phase 2 constraints:
/// - Single font
/// - No shaping
/// - Metrics only
pub struct LabelFont {
    font: Font,
    metrics: FontMetrics,
}

impl LabelFont {
    /// Load the embedded label font.
    pub fn load() -> Self {
        // Embedded font bytes
        let font_bytes = include_bytes!("../assets/fonts/Inter-Regular.ttf");

        let font = Font::from_bytes(font_bytes as &[u8], fontdue::FontSettings::default())
            .expect("Failed to load embedded label font");

        let metrics = Self::compute_metrics(&font);

        Self { font, metrics }
    }

    /// Access font metrics.
    pub fn metrics(&self) -> FontMetrics {
        self.metrics
    }

    /// Internal: compute normalized font metrics.
    fn compute_metrics(font: &Font) -> FontMetrics {
        // Use the same canonical raster size everywhere in the regular text pipeline.
        let px = LABEL_FONT_RASTER_PX;

        let m = font.metrics('H', px);

        // `height` is the actual rasterized pixel height.
        let cap_height = m.height as f32;
        let ascent = cap_height + m.ymin.max(0) as f32;
        let descent = (-m.ymin).max(0) as f32;

        FontMetrics {
            raster_px: px,
            cap_height,
            ascent,
            descent,
            line_height: ascent + descent,
        }
    }

    /// Access underlying font (used later for glyph rasterization).
    pub(crate) fn font(&self) -> &Font {
        &self.font
    }
}
