use std::sync::Arc;
use crate::resource::text::atlas::GlyphAtlas;
use crate::resource::text::layout::LabelLayout;
use crate::backend::renderer::mesh::{Mesh, MeshData};
use crate::backend::renderer::vertex::text::TextVertex;

/// Generates a GPU-ready Mesh from a high-level LabelLayout.
/// This acts as the "Baker" for text geometry.
pub fn build_label_mesh(
    layout: &LabelLayout,
    atlas: &GlyphAtlas,
) -> Arc<Mesh> {
    let mut vertices: Vec<TextVertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    let mut index_offset: u16 = 0;

    // World-unit scaling
    let scale_y = layout.height / atlas.cap_height_px;
    let scale_x = scale_y;

    for glyph in &layout.glyphs {
        let info = match atlas.glyphs.get(&glyph.ch) {
            Some(info) => info,
            None => continue,
        };

        let w = info.size_px[0] as f32 * scale_x;
        let h = info.size_px[1] as f32 * scale_y;

        let x0 = glyph.x + info.bearing_px[0] as f32 * scale_x;
        let x1 = x0 + w;
        let y1 = info.bearing_px[1] as f32 * scale_y;
        let y0 = y1 - h;

        let uv_min = info.uv_min;
        let uv_max = info.uv_max;

        vertices.extend_from_slice(&[
            TextVertex { position: [x0, y0, 0.0], uv: [uv_min[0], uv_max[1]] },
            TextVertex { position: [x1, y0, 0.0], uv: [uv_max[0], uv_max[1]] },
            TextVertex { position: [x1, y1, 0.0], uv: [uv_max[0], uv_min[1]] },
            TextVertex { position: [x0, y1, 0.0], uv: [uv_min[0], uv_min[1]] },
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
