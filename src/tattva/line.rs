// src/tattva/line.rs

use crate::renderer::mesh::Mesh;
use crate::tattva::{Tattva, TattvaProps};
use crate::transform::Transform;
use std::any::Any;
use std::sync::Arc;

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f32,
    pub color: Option<[f32; 3]>,
    mesh: Arc<Mesh>,
}

impl Line {
    pub fn new(props: TattvaProps) -> Self {
        let start = props.start.unwrap_or(Vec3::ZERO);
        let end = props.end.unwrap_or(Vec3::X);
        let thickness = props.thickness.unwrap_or(0.02);
        let color = props.color.unwrap_or([1.0, 1.0, 1.0]);
        let mesh = Mesh::line(start.into(), end.into(), thickness, color);
        Self {
            start,
            end,
            thickness,
            color: props.color,
            mesh,
        }
    }
}

impl Tattva for Line {
    fn mesh(&self) -> Arc<Mesh> {
        self.mesh.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self // Correctly casts the concrete self reference to &dyn Any
    }
}
