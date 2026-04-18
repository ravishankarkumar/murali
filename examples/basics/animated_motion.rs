use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    scene.add_tattva(
        Label::new("Animated Motion", 0.32).with_color(WHITE),
        3.0 * UP,
    );

    // Content
    let square_id = scene.add_tattva(
        Square::new(1.2, RED_B),
        4.0 * LEFT,
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.65, 48, GREEN_D),
        Vec3::new(4.0, -1.5, 0.0),
    );

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // Timeline
    let mut timeline = Timeline::new();
    timeline
        .animate(square_id)
        .at(0.0)
        .for_duration(2.2)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(2.6, 0.8, 0.0))
        .spawn();

    timeline
        .animate(circle_id)
        .at(0.4)
        .for_duration(2.6)
        .ease(Ease::OutQuad)
        .move_to(Vec3::new(-2.5, 1.3, 0.0))
        .spawn();

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}
