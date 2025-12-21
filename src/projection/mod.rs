pub mod context;
pub mod primitives;

pub use context::ProjectionCtx;
pub use primitives::RenderPrimitive;

pub trait Project: Send + Sync { // Added Send + Sync for safety
    fn project(&self, ctx: &mut ProjectionCtx);
}