use glam::Vec4;

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};

/// The Frontend LaTeX object. 
/// Pure semantic intent. No file IO occurs here.
pub struct Latex {
    pub source: String,
    pub world_height: f32,
    pub color: Vec4,
}

impl Latex {
    /// Creates a new LaTeX Tattva from a raw string.
    /// This is a fast, pure operation.
    pub fn new<S: Into<String>>(source: S, world_height: f32) -> Self {
        Self {
            source: source.into(),
            world_height,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

impl Project for Latex {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We emit the raw source and height.
        // The Sync Boundary will receive this and check the Resource Layer
        // (resource/latex/) to see if a cached texture already exists 
        // for this string. If not, IT will trigger the Tectonic compiler.
        ctx.emit(RenderPrimitive::Latex {
            source: self.source.clone(),
            height: self.world_height,
            color: self.color,
            offset: glam::Vec3::ZERO,
        });
    }
}

impl Bounded for Latex {
    fn local_bounds(&self) -> Bounds {
        let width = self.source.chars().count() as f32 * self.world_height * 0.55;
        Bounds::from_center_size(glam::Vec2::ZERO, glam::vec2(width.max(self.world_height), self.world_height))
    }
}
