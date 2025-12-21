// src/tattva/circle.rs

use crate::renderer::mesh::Mesh;
use crate::tattva::{Tattva, TattvaProps};
use crate::transform::Transform;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: f32,
    pub segments: u32,
    pub color: Option<[f32; 3]>,
    mesh: Arc<Mesh>,
}

impl Circle {
    pub fn new(props: TattvaProps) -> Self {
        let radius = props.radius.unwrap_or(1.0);
        let segments = props.segments.unwrap_or(32);
        let color = props.color.unwrap_or([1.0, 1.0, 1.0]);
        let mesh = Mesh::circle(radius, segments, color);
        Self {
            radius,
            segments,
            color: props.color,
            mesh,
        }
    }
}

impl Tattva for Circle {
    fn mesh(&self) -> Arc<Mesh> {
        self.mesh.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self // Correctly casts the concrete self reference to &dyn Any
    }
}
