use glam::Vec4;

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};

/// The Frontend LaTeX object.
/// Pure semantic intent. No file IO occurs here.
pub struct Latex {
    pub source: String,
    pub world_height: f32,
    pub color: Vec4,
    /// Character reveal progress: 0.0 = no characters, 1.0 = all characters
    pub char_reveal: f32,
    /// Reveal mode: true = typewriter (fixed position), false = reveal (shifting)
    pub typewriter_mode: bool,
}

impl Latex {
    /// Creates a new LaTeX Tattva from a raw string.
    /// This is a fast, pure operation.
    pub fn new<S: Into<String>>(source: S, world_height: f32) -> Self {
        Self {
            source: source.into(),
            world_height,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            char_reveal: 1.0,
            typewriter_mode: false, // Default to reveal mode
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    /// Get the revealed text based on char_reveal progress
    fn get_revealed_text(&self) -> String {
        let char_count = self.source.chars().count();
        let reveal_count = (char_count as f32 * self.char_reveal.clamp(0.0, 1.0)).ceil() as usize;
        self.source.chars().take(reveal_count).collect()
    }
}

impl Project for Latex {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We emit the raw source and height.
        // The Sync Boundary will receive this and check the Resource Layer
        // (resource/latex/) to see if a cached texture already exists
        // for this string. If not, IT will trigger the Tectonic compiler.
        let revealed_text = self.get_revealed_text();
        
        if self.typewriter_mode {
            // Typewriter mode: text grows from left to right, stays left-aligned
            // The mesh is centered, so we need to offset it to align the left edges
            let full_width = self.source.chars().count() as f32 * self.world_height * 0.55;
            let revealed_width = revealed_text.chars().count() as f32 * self.world_height * 0.55;
            
            // Offset to keep left edge aligned as text grows
            let offset_x = (revealed_width - full_width) / 2.0;
            
            ctx.emit(RenderPrimitive::Latex {
                source: revealed_text,
                height: self.world_height,
                color: self.color,
                offset: glam::Vec3::new(offset_x, 0.0, 0.0),
            });
        } else {
            // Reveal mode: text grows from center, stays centered
            // The mesh is already centered, so no offset needed
            ctx.emit(RenderPrimitive::Latex {
                source: revealed_text,
                height: self.world_height,
                color: self.color,
                offset: glam::Vec3::ZERO,
            });
        }
    }
}

impl Bounded for Latex {
    fn local_bounds(&self) -> Bounds {
        let width = self.source.chars().count() as f32 * self.world_height * 0.55;
        Bounds::from_center_size(
            glam::Vec2::ZERO,
            glam::vec2(width.max(self.world_height), self.world_height),
        )
    }
}
