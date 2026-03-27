use glam::{vec2, Vec4};
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::projection::Mesh;

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub radius_x: f32,
    pub radius_y: f32,
    pub segments: u32,
    pub color: Vec4,
}

impl Ellipse {
    pub fn new(radius_x: f32, radius_y: f32, color: Vec4) -> Self {
        Self {
            radius_x,
            radius_y,
            segments: 32,
            color,
        }
    }

    pub fn with_segments(mut self, segments: u32) -> Self {
        self.segments = segments;
        self
    }
}

impl Project for Ellipse {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let mesh = Mesh::ellipse(self.radius_x, self.radius_y, self.segments, self.color);
        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}

impl Bounded for Ellipse {
    fn local_bounds(&self) -> Bounds {
        Bounds::new(
            vec2(-self.radius_x, -self.radius_y),
            vec2(self.radius_x, self.radius_y),
        )
    }
}
