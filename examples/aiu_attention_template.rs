use glam::Vec3;
use murali::engine::scene::Scene;
use murali::frontend::collection::ai::templates::AiUnderTheHoodTemplates;
use murali::frontend::collection::text::label::Label;
use murali::frontend::theme::Theme;
use murali::frontend::Tattva;
use murali::App;

fn add_tattva<T>(scene: &mut Scene, state: T, position: Vec3) -> usize
where
    T: murali::projection::Project + murali::frontend::layout::Bounded + Send + Sync + 'static,
{
    let tattva = Tattva::new(0, state);
    let id = scene.add(tattva);

    if let Some(t) = scene.get_tattva_any_mut(id) {
        let mut props = t.props().write();
        props.position = position;
    }

    id
}

fn main() -> anyhow::Result<()> {
    let theme = Theme::ai_under_the_hood();
    let mut scene = Scene::new();

    add_tattva(
        &mut scene,
        Label::new("AI Under The Hood: attention template", 0.30).with_color(theme.text_primary),
        Vec3::new(0.0, 3.2, 0.0),
    );

    add_tattva(
        &mut scene,
        AiUnderTheHoodTemplates::token_sequence(vec!["the", "model", "attends"], 0.28),
        Vec3::new(0.0, 2.0, 0.0),
    );

    add_tattva(
        &mut scene,
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

    add_tattva(
        &mut scene,
        AiUnderTheHoodTemplates::transformer_block(),
        Vec3::new(2.6, -0.4, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);
    App::new()?.with_scene(scene).run_app()
}
