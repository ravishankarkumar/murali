pub mod context;
pub mod primitives;

pub use context::ProjectionCtx;
pub use primitives::RenderPrimitive;

pub trait Project: Send + Sync {
    fn project(&self, ctx: &mut ProjectionCtx);
}
