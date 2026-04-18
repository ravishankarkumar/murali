use crate::projection::ProjectionCtx;

pub trait Indicate {
    /// Projects the tattva with an indication effect (glow, pulse, internal signal).
    /// `t` is the progress of the indication event (0.0 to 1.0).
    fn project_indicated(&self, ctx: &mut ProjectionCtx, t: f32);
}
