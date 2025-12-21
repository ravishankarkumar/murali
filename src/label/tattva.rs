use std::any::Any;

use crate::tattva::Tattva;

/// Semantic tattva for lightweight, non-math text.
///
/// Intended use cases:
/// - Axis tick labels
/// - Numeric annotations
/// - Short labels attached to objects
///
/// Design constraints (Phase 2):
/// - Single-line only
/// - World-space sizing
/// - No paragraphs or wrapping
/// - Fast to create many instances
#[derive(Debug, Clone)]
pub struct LabelTattva {
    /// UTF-8 text (initially ASCII-focused)
    pub text: String,

    /// Height of the label in world units
    ///
    /// This defines the visual size of the text in the scene.
    /// Raster / glyph resolution is decided later by the renderer.
    pub world_height: f32,
}

impl LabelTattva {
    /// Create a new label with the given text and world-space height.
    pub fn new<S: Into<String>>(text: S, world_height: f32) -> Self {
        Self {
            text: text.into(),
            world_height,
        }
    }
}

impl Tattva for LabelTattva {
    fn mesh(&self) -> std::sync::Arc<crate::renderer::mesh::Mesh> {
        // Labels are materialized later (glyph layout → quad mesh),
        // so asking for a mesh at this stage is a logic error.
        panic!(
            "LabelTattva mesh requested before materialization. \
             Label geometry is created during App::materialize_scene()."
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
