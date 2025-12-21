// src/tattva/cube.rs
use crate::renderer::mesh::Mesh;
use crate::tattva::Tattva;
use crate::tattva::TattvaProps;
use crate::transform::Transform;
use crate::renderer::mesh::MeshInstance;
use std::any::Any;
use std::sync::Arc;

/// Simple Cube tattva: geometry parameters are read from TattvaProps::size
/// and placement from TattvaProps::transform.
#[derive(Debug, Clone)]
pub struct Cube {
    pub size: f32,
    pub transform: Transform,
    /// optional visual meta (kept for future use)
    pub color: Option<[f32; 3]>,
    mesh: Arc<Mesh>,
}

impl Cube {
    /// Construct a cube from a TattvaProps object.
    /// If props.size is None, default to 1.0.
    pub fn new(props: TattvaProps) -> Self {
        let size = props.size.unwrap_or(1.0);
        let color = props.color.unwrap_or([1.0, 1.0, 1.0]);
        let mesh = Mesh::cube(size, color);
        Self {
            size,
            transform: props.transform,
            color: props.color,
            mesh,
        }
    }
}

impl Tattva for Cube {
    fn mesh(&self) -> Arc<Mesh> {
        self.mesh.clone()
    }
    
    fn as_any(&self) -> &dyn Any {
        self // Correctly casts the concrete self reference to &dyn Any
    }
}
