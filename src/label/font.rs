use fontdue::Font;

/// Font metrics normalized to font units.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
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
        // Use a canonical font size for metric extraction.
        // We will scale later into world space.
        let px = 100.0;

        let m = font.metrics('H', px);

        // fontdue gives ascent/descent via bounding box + baseline
        let ascent = m.bounds.height + m.bounds.ymin.abs();
        let descent = m.bounds.ymin.abs();

        FontMetrics {
            cap_height: ascent,
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
