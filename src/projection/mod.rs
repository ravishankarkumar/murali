pub mod style;
pub mod context;
pub mod mesh;
pub mod primitives;

pub use context::ProjectionCtx;
pub use mesh::{Mesh, MeshData};
pub use primitives::RenderPrimitive;

pub trait Project: Send + Sync {
    fn project(&self, ctx: &mut ProjectionCtx);
}
