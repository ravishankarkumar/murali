use glam::{Vec2, Vec3, Vec4};
use crate::scene::SharedProps;

pub struct LineComponent {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f32,
}

pub struct QuadComponent {
    pub size: Vec2,
    pub texture_id: Option<u64>,
}

pub struct ColorComponent(pub Vec4);

// We use the Sangh's shared props as a component directly.
// hecs allows any type that is Send + Sync + 'static.
pub type TransformComponent = SharedProps;