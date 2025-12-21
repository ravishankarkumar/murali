use crate::{
    renderer::renderer::Renderer,
    scene::Scene,
    config::RenderConfig,
};

pub struct ConstructContext<'a> {
    pub renderer: &'a mut Renderer,
    pub scene: &'a mut Scene,
    pub render_config: &'a RenderConfig,
}
