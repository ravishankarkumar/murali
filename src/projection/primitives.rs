use crate::projection::mesh::Mesh;
use glam::{Vec3, Vec4};
use std::sync::Arc;

/// CPU-side render primitives produced by the Projection layer.
/// These are later materialized into GPU resources by the Backend.
pub enum RenderPrimitive {
    /// CPU-side triangle mesh (to be uploaded to GPU later)
    Mesh(Arc<Mesh>),

    /// Line primitive (may be rendered as instanced quad or mesh)
    Line {
        start: Vec3,
        end: Vec3,
        thickness: f32,
        color: Vec4,
        dash_length: f32,
        gap_length: f32,
        dash_offset: f32,
    },

    /// Text primitive (resolved by text system)
    Text {
        content: String,
        height: f32,
        color: Vec4,
        offset: Vec3,
    },

    /// LaTeX primitive (resolved by LaTeX system)
    Latex {
        source: String,
        height: f32,
        color: Vec4,
        offset: Vec3,
    },

    /// Typst primitive (resolved by Typst system)
    Typst {
        source: String,
        height: f32,
        color: Vec4,
        offset: Vec3,
    },
}
