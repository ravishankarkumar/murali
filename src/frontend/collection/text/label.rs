
use glam::Vec4;
use crate::frontend::layout::{Bounded, Bounds};
use crate::resource::text::layout::measure_label;
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};

/// Frontend Label. 
/// Lightweight text intended for UI, ticks, or annotations.
///
/// This represents the *intent* to show text. The actual glyph generation 
/// happens in the Projection/Backend boundary using the Resource Layer.
#[derive(Debug, Clone)]
pub struct Label {
    pub text: String,
    pub world_height: f32,
    pub color: Vec4,
}

impl Label {
    /// Creates a new Label with specified text and world-space height.
    pub fn new<S: Into<String>>(text: S, world_height: f32) -> Self {
        Self {
            text: text.into(),
            world_height,
            color: Vec4::new(1.0, 1.0, 1.0,1.0), // Default to white
        }
    }

    /// Builder-style method to set color.
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

impl Project for Label {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We emit a primitive that describes the requirement.
        // The Backend's Sync Boundary will use resource/text/layout.rs 
        // to convert this string into renderable glyph quads.
        ctx.emit(RenderPrimitive::Text {
            content: self.text.clone(),
            height: self.world_height,
            color: self.color,
            offset: glam::Vec3::ZERO,
        });
    }
}

impl Bounded for Label {
    fn local_bounds(&self) -> Bounds {
        let layout = measure_label(&self.text, self.world_height);
        Bounds::from_center_size(
            glam::Vec2::ZERO,
            glam::vec2(layout.width.max(self.world_height * 0.4), layout.height),
        )
    }
}
