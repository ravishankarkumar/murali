use glam::{Vec2, Vec4};
use crate::frontend::layout::{Bounded, Bounds};
use crate::projection::{Project, ProjectionCtx, RenderPrimitive};
use crate::projection::Mesh;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub vertices: Vec<Vec2>,
    pub color: Vec4,
}

impl Polygon {
    pub fn new(vertices: Vec<Vec2>, color: Vec4) -> Self {
        Self {
            vertices,
            color,
        }
    }

    /// Creates a regular polygon with n sides.
    pub fn regular(n: usize, radius: f32, color: Vec4) -> Self {
        let mut vertices = Vec::with_capacity(n);
        for i in 0..n {
            let t = (i as f32 / n as f32) * std::f32::consts::TAU;
            vertices.push(glam::vec2(radius * t.cos(), radius * t.sin()));
        }
        Self::new(vertices, color)
    }
}

impl Project for Polygon {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let mesh = Mesh::polygon(self.vertices.clone(), self.color);
        ctx.emit(RenderPrimitive::Mesh(mesh));
    }
}

impl Bounded for Polygon {
    fn local_bounds(&self) -> Bounds {
        if self.vertices.is_empty() {
            return Bounds::default();
        }
        let mut min = glam::vec2(f32::MAX, f32::MAX);
        let mut max = glam::vec2(f32::MIN, f32::MIN);
        for p in &self.vertices {
            min = min.min(*p);
            max = max.max(*p);
        }
        Bounds::new(min, max)
    }
}
