// src/renderer/mesh/latex_quad.rs

use std::sync::Arc;

use crate::renderer::mesh::{Mesh, MeshData};
use crate::renderer::vertex::text::TextVertex;

/// Build a world-space quad from raster dimensions.
///
/// - Preserves aspect ratio
/// - Uses world_height as the authoritative dimension
/// - UV origin = top-left (matches resvg output)
pub fn build_textured_quad(
    width_px: u32,
    height_px: u32,
    world_height: f32,
) -> Arc<Mesh> {
    let w_px = width_px as f32;
    let h_px = height_px as f32;

    let scale = world_height / h_px;
    let world_width = w_px * scale;

    let hw = world_width * 0.5;
    let hh = world_height * 0.5;

    let vertices = vec![
        // bottom-left
        TextVertex {
            position: [-hw, -hh, 0.0],
            uv: [0.0, 1.0],
        },
        // bottom-right
        TextVertex {
            position: [ hw, -hh, 0.0],
            uv: [1.0, 1.0],
        },
        // top-right
        TextVertex {
            position: [ hw,  hh, 0.0],
            uv: [1.0, 0.0],
        },
        // top-left
        TextVertex {
            position: [-hw,  hh, 0.0],
            uv: [0.0, 0.0],
        },
    ];

    let indices = vec![
        0, 1, 2,
        0, 2, 3,
    ];

    Arc::new(Mesh {
        data: MeshData::Text(vertices),
        indices,
    })
}
