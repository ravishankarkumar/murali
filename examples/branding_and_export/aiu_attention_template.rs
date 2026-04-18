use glam::Vec3;
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::ai::templates::AiUnderTheHoodTemplates;
use murali::frontend::collection::text::label::Label;
use murali::frontend::theme::Theme;

fn main() -> anyhow::Result<()> {
    let theme = Theme::ai_under_the_hood();
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("AI Under The Hood: attention template", 0.30).with_color(theme.text_primary),
        Vec3::new(0.0, 3.2, 0.0),
    );

    scene.add_tattva(
        AiUnderTheHoodTemplates::token_sequence(vec!["the", "model", "attends"], 0.28),
        2.0 * UP,
    );

    scene.add_tattva(
        AiUnderTheHoodTemplates::attention_matrix(
            vec![
                vec![0.70, 0.20, 0.10],
                vec![0.22, 0.58, 0.20],
                vec![0.12, 0.21, 0.67],
            ],
            Some(vec!["the".into(), "model".into(), "attends".into()]),
        ),
        Vec3::new(-2.4, -0.6, 0.0),
    );

    scene.add_tattva(
        AiUnderTheHoodTemplates::transformer_block(),
        Vec3::new(2.6, -0.4, 0.0),
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    App::new()?.with_scene(scene).run_app()
}
