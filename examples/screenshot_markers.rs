use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::utility::screenshot_marker::ScreenshotMarker;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Screenshot Markers", 0.34).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::new(0.0, 3.0, 0.0),
    );

    let square_id = scene.add_tattva(
        Square::new(1.2, Vec4::new(0.93, 0.36, 0.30, 1.0)),
        Vec3::new(-3.0, 0.0, 0.0),
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.7, 48, Vec4::new(0.23, 0.70, 0.40, 1.0)),
        Vec3::new(3.0, 0.0, 0.0),
    );

    let first_capture = scene.add_tattva(ScreenshotMarker::new("captures/step_01.png"), Vec3::ZERO);
    let second_capture =
        scene.add_tattva(ScreenshotMarker::new("captures/step_02.png"), Vec3::ZERO);

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    let mut timeline = Timeline::new();
    timeline
        .animate(square_id)
        .at(0.0)
        .for_duration(1.4)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(-0.8, 0.0, 0.0))
        .spawn();

    timeline
        .animate(first_capture)
        .at(0.7)
        .for_duration(0.0)
        .capture_frame()
        .spawn();

    timeline
        .animate(circle_id)
        .at(1.4)
        .for_duration(1.4)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(1.0, 1.2, 0.0))
        .spawn();

    timeline
        .animate(second_capture)
        .at(2.2)
        .for_duration(0.0)
        .capture_frame()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
