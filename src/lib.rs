pub mod core;
pub mod ecs;
pub mod sangh;
pub mod projection;
pub mod math;
pub mod renderer;

// pub mod prelude {
//     pub use crate::core::scene::Scene;
//     pub use crate::core::timeline::{Timeline, Easing, ScheduledAnimation};
//     pub use crate::sangh::{Sangh, Project, DrawableProps};
//     pub use crate::projection::{ProjectionCtx, RenderPrimitive};
//     pub use glam::{Vec2, Vec3, Vec4, Quat};
// }

pub mod prelude {
    pub use crate::core::scene::Scene;
    pub use crate::core::timeline::{Timeline, Easing, ScheduledAnimation};
    pub use crate::sangh::{Sangh, DrawableProps};
    pub use crate::projection::Project; // Re-export directly from projection
    pub use crate::projection::context::ProjectionCtx;
    pub use crate::projection::primitives::RenderPrimitive;
    pub use glam::{Vec2, Vec3, Vec4, Quat};
}