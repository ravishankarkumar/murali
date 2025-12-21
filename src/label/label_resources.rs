// src/label/label_resources.rs

use std::sync::Arc;

use crate::renderer::renderer::Renderer;

pub struct LabelResources {
    pub(crate) font: crate::label::font::LabelFont,
    pub(crate) atlas: crate::label::atlas::GlyphAtlas,
    pub(crate) bind_group: Arc<wgpu::BindGroup>,
}

pub(crate) fn ensure_label_resources<'a>(
    label_resources: &'a mut Option<LabelResources>,
    renderer: &mut Renderer,
) -> &'a LabelResources {
    if label_resources.is_none() {
        use crate::label::atlas::GlyphAtlas;
        use crate::label::font::LabelFont;

        let font = LabelFont::load();
        let atlas = GlyphAtlas::build(&font);

        let bind_group = Arc::new(
            renderer.create_text_bind_group_from_raster(
                &atlas.rgba,
                atlas.width,
                atlas.height,
            ),
        );

        *label_resources = Some(LabelResources {
            font,
            atlas,
            bind_group,
        });
    }

    label_resources.as_ref().unwrap()
}
