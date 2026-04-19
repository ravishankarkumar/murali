use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::export::{ExportSettings, export_scene};
use murali::engine::scene::Scene;
use murali::frontend::collection::ai::neural_network_diagram::{
    ActivationFunc, NeuralNetworkDiagram,
};
use murali::frontend::collection::text::code_block::CodeBlock;
use murali::frontend::collection::text::label::Label;
use murali::frontend::theme::Theme;

fn build_scene() -> Scene {
    let theme = Theme::ai_under_the_hood();
    let mut scene = Scene::new();

    // Title
    scene.add_tattva(
        Label::new("AI Under The Hood: New Features!", 0.40).with_color(theme.text_primary),
        Vec3::new(0.0, 3.2, 0.0),
    );

    // Neural Network with labels and activation
    let nn = NeuralNetworkDiagram::new(vec![3, 4, 2])
        .with_labels(vec!["Input", "Hidden", "Output"])
        .with_activation(ActivationFunc::ReLU);

    scene.add_tattva(nn, Vec3::new(-3.5, -0.5, 0.0));

    // Code Block
    let code = "fn animate(t: f32) {\n  let pos = lerp(a, b, t);\n  draw(pos);\n}";
    let cb = CodeBlock::new(code, "rust", 0.9).with_color(BLUE_A);

    scene.add_tattva(cb, Vec3::new(2.5, -0.5, 0.0));

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene
}

fn main() -> anyhow::Result<()> {
    let theme = Theme::ai_under_the_hood();
    let scene = build_scene();

    // Check if we should export or just run the app
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--export".to_string()) {
        println!("Exporting to GIF...");
        let settings = ExportSettings {
            duration_seconds: 2.0,
            output_dir: "render_output/blog_showcase_frames".into(),
            basename: "blog_showcase".to_string(),
            gif_path: Some("render_output/blog_showcase.gif".into()),
            video_path: None,
            clear_color: theme.background,
            ..ExportSettings::default()
        };
        export_scene(scene, &settings)?;
        println!("Export complete: render_output/blog_showcase.gif");
    } else {
        App::new()?.with_scene(scene).run_app()?;
    }

    Ok(())
}
