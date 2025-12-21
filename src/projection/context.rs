use crate::engine::scene::SharedProps;
use crate::projection::RenderPrimitive;

/// The Projection Context is the "Collector".
/// Each Tattva is passed this context during the project() phase.
pub struct ProjectionCtx {
    /// Primitives collected during this projection pass
    pub primitives: Vec<RenderPrimitive>,

    /// Shared spatial properties of the parent Tattva
    pub props: SharedProps,
}

impl ProjectionCtx {
    pub fn new(props: SharedProps) -> Self {
        Self {
            primitives: Vec::new(),
            props,
        }
    }

    /// Emit a renderable primitive
    pub fn emit(&mut self, primitive: RenderPrimitive) {
        self.primitives.push(primitive);
    }
}
