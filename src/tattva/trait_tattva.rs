// -----------------------------
// file: src/tattva/trait_tattva.rs
// -----------------------------

use std::any::Any;
use std::sync::Arc;

// point at the mesh module file
use crate::renderer::mesh::Mesh;
use crate::transform::Transform;

/// The Tattva trait: every drawable primitive implements this.
pub trait Tattva: Any + Send + Sync {
    /// Return the mesh definition (shared, immutable).
    fn mesh(&self) -> Arc<Mesh>;

    /// Allow downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Extension helper trait
pub trait TattvaExt: Tattva {
    fn into_mesh_arc(self: Box<Self>) -> Arc<Mesh>
    where
        Self: Sized,
    {
        self.mesh()
    }
}

impl<T: Tattva + ?Sized> TattvaExt for T {}
