use glam::Vec4;

use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::backend::renderer::mesh::Mesh;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Cube {
    pub size: f32,
    pub color: Vec4,
}

impl Cube {
    /// Pure semantic constructor. 
    /// The transform is now handled by the Tattva wrapper, not this struct.
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

impl Project for Cube {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We generate the Cube mesh geometry on demand.
        // The color is baked into the vertex data for this primitive.
        let mesh = Mesh::cube(
            self.size, 
            self.color
        );

        // Emit the mesh primitive to the Projection Context
        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}