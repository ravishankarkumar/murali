use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::export::{ExportSettings, export_scene};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    // Title
    scene.add_tattva(
        Label::new("Scene Capture Schedules", 0.34).with_color(WHITE),
        3.0 * UP,
    );

    // Subtitle
    scene.add_tattva(
        Label::new(
            "Screenshots and GIFs scheduled from authored capture helpers",
            0.18,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, 2.45, 0.0),
    );

    // Content
    let square_id = scene.add_tattva(
        Square::new(1.2, RED_B),
        3.0 * LEFT,
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.7, 48, GREEN_C),
        3.0 * RIGHT,
    );

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // Timeline
    let mut timeline = Timeline::new();
    timeline
        .animate(square_id)
        .at(0.0)
        .for_duration(1.4)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(-0.8, 0.0, 0.0))
        .spawn();

    timeline
        .animate(circle_id)
        .at(1.4)
        .for_duration(1.4)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(1.0, 1.2, 0.0))
        .spawn();

    scene.play(timeline);

    // Captures
    scene.capture_screenshots_named([
        (0.7, Some("captures/step_01.png")),
        (2.2, None::<&str>),
        (2.7, Some("captures/step_03.png")),
    ]);
    scene.capture_gif("movement_overview", [0.7, 2.2, 2.7]);
    scene.capture_gif("square_focus", [0.7]);
    scene.capture_gif("circle_focus", [2.2, 2.7]);

    scene
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--export") {
        let settings = ExportSettings {
            duration_seconds: 2.8,
            output_dir: "render_output/screenshot_marker_frames".into(),
            basename: "screenshot_markers".to_string(),
            video_path: None,
            gif_path: None,
            capture_gif_dir: Some("render_output/screenshot_marker_gifs".into()),
            ..ExportSettings::default()
        };
        return export_scene(build_scene(), &settings);
    }

    App::new()?.with_scene(build_scene()).run_app()
}
