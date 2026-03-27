use std::sync::Arc;

use crate::backend::renderer::vertex::text::TextVertex;
use crate::projection::{Mesh, MeshData};

/// Build a world-space textured quad mesh suitable for text rendering.
///
/// - `width` and `height` are in WORLD UNITS
/// - Quad is centered at origin
/// - UVs are standard (top-left = (0,0))
pub fn make_textured_quad_mesh_for_raster(
    width: f32,
    height: f32,
) -> Arc<Mesh> {
    let hw = width * 0.5;
    let hh = height * 0.5;

    let vertices = vec![
        TextVertex { position: [-hw, -hh, 0.0], uv: [0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
        TextVertex { position: [ hw, -hh, 0.0], uv: [1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0] },
        TextVertex { position: [ hw,  hh, 0.0], uv: [1.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
        TextVertex { position: [-hw,  hh, 0.0], uv: [0.0, 0.0], color: [1.0, 1.0, 1.0, 1.0] },
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    Arc::new(Mesh {
        data: MeshData::Text(vertices),
        indices,
    })
}
