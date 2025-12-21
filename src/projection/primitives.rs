use glam::{Vec2, Vec3, Vec4};

/// The "Dumb" data that the Renderer actually understands.
/// These are backend-agnostic.
#[derive(Debug, Clone)]
pub enum RenderPrimitive {
    Line {
        start: Vec3,
        end: Vec3,
        thickness: f32,
        color: Vec4,
    },
    Quad {
        size: Vec2,
        texture_id: Option<u64>, // For LaTeX/Typst textures
        color: Vec4,
    },
    // We can add ParticleBatch, Mesh, etc. later
}