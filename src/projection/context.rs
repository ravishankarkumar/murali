use super::primitives::RenderPrimitive;

/// Collected by the Sync Boundary during the project() phase.
pub struct ProjectionCtx {
    pub primitives: Vec<RenderPrimitive>,
}

impl ProjectionCtx {
    pub fn new() -> Self {
        Self { primitives: Vec::new() }
    }

    pub fn add(&mut self, primitive: RenderPrimitive) {
        self.primitives.push(primitive);
    }
}