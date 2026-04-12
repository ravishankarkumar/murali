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

impl RenderPrimitive {
    pub fn translated(self, offset: Vec3) -> Self {
        match self {
            RenderPrimitive::Mesh(mesh) => RenderPrimitive::Mesh(mesh.as_ref().translated(offset)),
            RenderPrimitive::Line {
                start,
                end,
                thickness,
                color,
                dash_length,
                gap_length,
                dash_offset,
            } => RenderPrimitive::Line {
                start: start + offset,
                end: end + offset,
                thickness,
                color,
                dash_length,
                gap_length,
                dash_offset,
            },
            RenderPrimitive::Text {
                content,
                height,
                color,
                offset: old_offset,
            } => RenderPrimitive::Text {
                content,
                height,
                color,
                offset: old_offset + offset,
            },
            RenderPrimitive::Latex {
                source,
                height,
                color,
                offset: old_offset,
            } => RenderPrimitive::Latex {
                source,
                height,
                color,
                offset: old_offset + offset,
            },
            RenderPrimitive::Typst {
                source,
                height,
                color,
                offset: old_offset,
            } => RenderPrimitive::Typst {
                source,
                height,
                color,
                offset: old_offset + offset,
            },
        }
    }

    pub fn with_opacity(self, alpha: f32) -> Self {
        let f = |c: Vec4| Vec4::new(c.x, c.y, c.z, c.w * alpha);
        match self {
            RenderPrimitive::Mesh(mesh) => {
                // Mesh vertices have baked colors. Scaling opacity here requires re-tessellation
                // or a shader uniform. For now, we return it as is or handle it at projection.
                // TODO: support global alpha in Mesh
                RenderPrimitive::Mesh(mesh)
            }
            RenderPrimitive::Line {
                start,
                end,
                thickness,
                color,
                dash_length,
                gap_length,
                dash_offset,
            } => RenderPrimitive::Line {
                start,
                end,
                thickness,
                color: f(color),
                dash_length,
                gap_length,
                dash_offset,
            },
            RenderPrimitive::Text {
                content,
                height,
                color,
                offset,
            } => RenderPrimitive::Text {
                content,
                height,
                color: f(color),
                offset,
            },
            RenderPrimitive::Latex {
                source,
                height,
                color,
                offset,
            } => RenderPrimitive::Latex {
                source,
                height,
                color: f(color),
                offset,
            },
            RenderPrimitive::Typst {
                source,
                height,
                color,
                offset,
            } => RenderPrimitive::Typst {
                source,
                height,
                color: f(color),
                offset,
            },
        }
    }
}
