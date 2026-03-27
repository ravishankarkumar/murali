use glam::Vec4;

use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::projection::Mesh;

#[derive(Debug, Clone)]
pub struct Square {
    pub size: f32,
    pub color: Vec4,
}

impl Square {
    /// Pure semantic constructor.
    pub fn new(size: f32, color: Vec4) -> Self {
        Self {
            size,
            color,
        }
    }

    pub fn default_unit() -> Self {
        Self::new(1.0, Vec4::new(1.0, 1.0, 1.0, 1.0))
    }
}

impl Project for Square {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We generate the 2D square mesh on demand.
        // Note: size here usually refers to the side length.
        let mesh = Mesh::square(
            self.size, 
            self.color
        );

        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}

impl Bounded for Square {
    fn local_bounds(&self) -> Bounds {
        let half = self.size * 0.5;
        Bounds::new(glam::vec2(-half, -half), glam::vec2(half, half))
    }
}
