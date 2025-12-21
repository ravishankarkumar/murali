// src/tattva/square.rs

use crate::renderer::mesh::Mesh;
use crate::tattva::{Tattva, TattvaProps};
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Square {
    pub size: f32,
    pub color: Option<[f32; 3]>,
    mesh: Arc<Mesh>,
}

impl Square {
    pub fn new(props: TattvaProps) -> Self {
        let size = props.size.unwrap_or(1.0);
        let color = props.color.unwrap_or([1.0, 1.0, 1.0]);
        let mesh = Mesh::square(size, color);
        Self {
            color: props.color,
            size,
            mesh,
        }
    }
}

impl Tattva for Square {
    fn mesh(&self) -> Arc<Mesh> {
        self.mesh.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self // Correctly casts the concrete self reference to &dyn Any
    }
}
