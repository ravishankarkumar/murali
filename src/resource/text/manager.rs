use crate::resource::text::atlas::{GlyphAtlas, GlyphInfo};
use crate::resource::text::font::LabelFont;

/// Renamed from FontResourceManager to match the 'app.rs' expectation.
/// This acts as the "Baker" for all text metrics in the engine.
pub struct LabelResources {
    pub font: LabelFont,
    pub atlas: GlyphAtlas,
}

impl LabelResources {
    /// Lazy-loads the default font and builds the glyph atlas.
    pub fn new() -> Self {
        let font = LabelFont::load();
        let atlas = GlyphAtlas::build(&font);

        Self { font, atlas }
    }

    /// Helper for the Layout engine to find where a character sits.
    pub fn get_glyph_metrics(&self, character: char) -> Option<&GlyphInfo> {
        self.atlas.glyphs.get(&character)
    }
}

/// The state managed by the engine's Resource Registry.
pub struct TextResourceState {
    pub manager: Option<LabelResources>,
}

impl TextResourceState {
    pub fn ensure_loaded(&mut self) -> &LabelResources {
        self.manager.get_or_insert_with(LabelResources::new)
    }
}
