use std::sync::Arc;
use crate::frontend::props::DrawableProps;
use crate::projection::ProjectionCtx;
use crate::frontend::{Tattva, TattvaId};
use crate::frontend::props::SharedProps;

/// Object-safe interface for all Tattvas.
/// Scene, Animation, and SyncBoundary talk ONLY through this.
pub trait TattvaTrait: Send + Sync {
    /// Borrow shared transform / drawable properties
    fn props(&self) -> &SharedProps;

    /// Structural dirtiness (geometry-level)
    fn is_dirty(&self) -> bool;

    /// Reset dirty flag after sync
    fn clear_dirty(&mut self);

    /// Identity
    fn set_id(&mut self, id: TattvaId);
    fn id(&self) -> TattvaId;

    /// Projection hook (virtual dispatch)
    fn project(&self, ctx: &mut ProjectionCtx);
}

/// Blanket implementation for the generic Tattva<T> struct.
/// This bridges concrete math types → dyn TattvaTrait.
impl<T> TattvaTrait for Tattva<T>
where
    T: crate::projection::Project + Send + Sync + 'static,
{
    fn props(&self) -> &SharedProps {
        &self.props
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    fn set_id(&mut self, id: TattvaId) {
        self.id = id;
    }

    fn id(&self) -> TattvaId {
        self.id
    }

    fn project(&self, ctx: &mut ProjectionCtx) {
        self.state.project(ctx);
    }
}
