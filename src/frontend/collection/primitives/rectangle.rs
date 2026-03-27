use glam::{vec2, Vec4};
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::projection::Mesh;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub color: Vec4,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, color: Vec4) -> Self {
        Self {
            width,
            height,
            color,
        }
    }
}

impl Project for Rectangle {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let mesh = Mesh::rectangle(self.width, self.height, self.color);
        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}

impl Bounded for Rectangle {
    fn local_bounds(&self) -> Bounds {
        let hw = self.width * 0.5;
        let hh = self.height * 0.5;
        Bounds::new(vec2(-hw, -hh), vec2(hw, hh))
    }
}
