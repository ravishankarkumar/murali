use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::resource::text::layout::measure_label;
use glam::Vec4;

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
    /// Character reveal progress: 0.0 = no characters, 1.0 = all characters
    pub char_reveal: f32,
    /// Reveal mode: true = typewriter (fixed position), false = reveal (shifting)
    pub typewriter_mode: bool,
}

impl Label {
    /// Creates a new Label with specified text and world-space height.
    pub fn new<S: Into<String>>(text: S, world_height: f32) -> Self {
        Self {
            text: text.into(),
            world_height,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0), // Default to white
            char_reveal: 1.0,
            typewriter_mode: false, // Default to reveal mode
        }
    }

    /// Builder-style method to set color.
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// Get the revealed text based on char_reveal progress
    fn get_revealed_text(&self) -> String {
        let char_count = self.text.chars().count();
        let reveal_count = (char_count as f32 * self.char_reveal.clamp(0.0, 1.0)).ceil() as usize;
        self.text.chars().take(reveal_count).collect()
    }
}

impl Project for Label {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We emit a primitive that describes the requirement.
        // The Backend's Sync Boundary will use resource/text/layout.rs
        // to convert this string into renderable glyph quads.
        let revealed_text = self.get_revealed_text();
        
        if self.typewriter_mode {
            // Typewriter mode: text grows from left to right, stays left-aligned
            // The mesh is centered, so we need to offset it to align the left edges
            let full_layout = measure_label(&self.text, self.world_height);
            let revealed_layout = measure_label(&revealed_text, self.world_height);
            
            // Offset to keep left edge aligned as text grows
            let offset_x = (revealed_layout.width - full_layout.width) / 2.0;
            
            ctx.emit(RenderPrimitive::Text {
                content: revealed_text,
                height: self.world_height,
                color: self.color,
                offset: glam::Vec3::new(offset_x, 0.0, 0.0),
            });
        } else {
            // Reveal mode: text grows from center, stays centered
            // The mesh is already centered, so no offset needed
            ctx.emit(RenderPrimitive::Text {
                content: revealed_text,
                height: self.world_height,
                color: self.color,
                offset: glam::Vec3::ZERO,
            });
        }
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
