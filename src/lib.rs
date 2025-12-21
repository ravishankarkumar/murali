pub mod core;
pub mod ecs;
pub mod sangh;
pub mod projection;
pub mod math;
pub mod renderer;
pub mod camera;
pub mod app;
pub mod draw;
pub mod config;
pub mod scene;
pub mod label;
pub mod animation;
pub mod latex;
pub mod typst;
pub mod tattva;
pub mod transform;
pub mod timeline;
pub mod layout;

pub mod prelude {
    pub use crate::scene::{Scene,DrawableProps};
    pub use crate::timeline::{AnimState, ScheduledHandle, ScheduledAnimation};
    pub use crate::sangh::Sangh;
    pub use crate::projection::Project;
    pub use crate::projection::context::ProjectionCtx;
    pub use crate::projection::primitives::RenderPrimitive;
    pub use glam::{Vec2, Vec3, Vec4, Quat};
}