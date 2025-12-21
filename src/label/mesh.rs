use std::sync::Arc;

use crate::label::atlas::GlyphAtlas;
use crate::label::layout::{GlyphInstance, LabelLayout};
use crate::renderer::mesh::{Mesh, MeshData};
use crate::renderer::vertex::text::TextVertex;

/// Build a quad mesh for a label from glyph layout and atlas.
///
/// Output:
/// - One quad per glyph
/// - Positions in world space
/// - UVs into the glyph atlas
pub fn build_label_mesh(
    layout: &LabelLayout,
    atlas: &GlyphAtlas,
) -> Arc<Mesh> {
    let mut vertices: Vec<TextVertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    let mut index_offset: u16 = 0;

    // World-space scale factor
    let scale_y = layout.height / atlas.cap_height_px();
    let scale_x = scale_y; // uniform scaling

    for glyph in &layout.glyphs {
        let info = match atlas.glyph(glyph.ch) {
            Some(info) => info,
            None => continue,
        };

        // Convert glyph bitmap size → world units
        let w = info.size_px[0] as f32 * scale_x;
        let h = info.size_px[1] as f32 * scale_y;

        let x0 = glyph.x;
        let x1 = glyph.x + w;

        // Baseline at y = 0
        let y0 = -h;
        let y1 = 0.0;

        let uv_min = info.uv_min;
        let uv_max = info.uv_max;

        vertices.extend_from_slice(&[
            TextVertex {
                position: [x0, y0, 0.0],
                uv: [uv_min[0], uv_max[1]],
            },
            TextVertex {
                position: [x1, y0, 0.0],
                uv: [uv_max[0], uv_max[1]],
            },
            TextVertex {
                position: [x1, y1, 0.0],
                uv: [uv_max[0], uv_min[1]],
            },
            TextVertex {
                position: [x0, y1, 0.0],
                uv: [uv_min[0], uv_min[1]],
            },
        ]);

        indices.extend_from_slice(&[
            index_offset,
            index_offset + 1,
            index_offset + 2,
            index_offset,
            index_offset + 2,
            index_offset + 3,
        ]);

        index_offset += 4;
    }

    Arc::new(Mesh {
        data: MeshData::Text(vertices),
        indices,
    })
}
