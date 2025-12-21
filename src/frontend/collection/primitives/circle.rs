use glam::Vec4;

use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::backend::renderer::mesh::{Mesh, MeshInstance};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
    pub segments: u32,
    pub color: Vec4,
}

impl Circle {
    /// Pure semantic constructor. No Mesh is created here.
    pub fn new(radius: f32, segments: u32, color: Vec4) -> Self {
        Self {
            radius,
            segments,
            color,
        }
    }

    /// Convenience default
    pub fn default_unit() -> Self {
        Self::new(1.0, 32, Vec4::new(1.0, 1.0, 1.0, 1.0))
    }
}

impl Project for Circle {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // 1. Build CPU-side geometry
        let mesh = Mesh::circle(
            self.radius,
            self.segments,
            self.color, // drop alpha for mesh
        );


        // 2. Emit backend-ready primitive
        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}
