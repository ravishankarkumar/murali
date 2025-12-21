use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::backend::renderer::mesh::Mesh;
use std::sync::Arc;
use glam::{Vec3, Vec4};

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f32,
    pub color: Vec4,
}

impl Line {
    /// Pure semantic constructor.
    pub fn new(start: Vec3, end: Vec3, thickness: f32, color: Vec4) -> Self {
        Self {
            start,
            end,
            thickness,
            color,
        }
    }

    /// Creates a line from the origin to a point
    pub fn to(end: Vec3) -> Self {
        Self::new(Vec3::ZERO, end, 0.02, Vec4::new(1.0, 1.0, 1.0, 1.0))
    }
}

impl Project for Line {
    fn project(&self, ctx: &mut ProjectionCtx) {
        // We generate the line mesh based on the semantic start/end points.
        // In the future, the backend might use a specialized LinePrimitive 
        // for better performance, but for now, we emit a Mesh.
        let mesh = Mesh::line(
            self.start.into(), 
            self.end.into(), 
            self.thickness, 
            self.color
        );

        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}