use glam::Vec3;
use murali::engine::export::{ExportSettings, export_scene};
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::ai::templates::AiUnderTheHoodTemplates;
use murali::frontend::collection::text::label::Label;
use murali::frontend::theme::Theme;

fn build_scene() -> Scene {
    let theme = Theme::ai_under_the_hood();
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Attention export template", 0.32).with_color(theme.text_primary),
        3.0 * UP,
    );
    scene.add_tattva(
        AiUnderTheHoodTemplates::token_sequence(vec!["query", "key", "value"], 0.30),
        Vec3::new(0.0, 1.95, 0.0),
    );
    scene.add_tattva(
        AiUnderTheHoodTemplates::attention_matrix(
            vec![
                vec![0.72, 0.18, 0.10],
                vec![0.24, 0.51, 0.25],
                vec![0.10, 0.28, 0.62],
            ],
            Some(vec!["q".into(), "k".into(), "v".into()]),
        ),
        Vec3::new(-2.3, -0.5, 0.0),
    );
    scene.add_tattva(
        AiUnderTheHoodTemplates::neural_network(vec![3, 4, 2]),
        Vec3::new(2.6, -0.4, 0.0),
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene
}

fn main() -> anyhow::Result<()> {
    let theme = Theme::ai_under_the_hood();
    let settings = ExportSettings {
        duration_seconds: 1.0,
        output_dir: "render_output/aiu_attention_frames".into(),
        basename: "aiu_attention".to_string(),
        video_path: Some("render_output/aiu_attention.mp4".into()),
        clear_color: theme.background,
        ..ExportSettings::default()
    };

    export_scene(build_scene(), &settings)
}
