use std::any::Any;
use std::sync::Arc;

use crate::renderer::mesh::Mesh;
use crate::tattva::Tattva;

/// Internal tattva used during materialization
/// when geometry is generated dynamically.
pub struct MeshOnlyTattva {
    mesh: Arc<Mesh>,
}

impl MeshOnlyTattva {
    pub fn new(mesh: Arc<Mesh>) -> Self {
        Self { mesh }
    }
}

impl Tattva for MeshOnlyTattva {
    fn mesh(&self) -> Arc<Mesh> {
        self.mesh.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
