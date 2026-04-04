use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // The Source (starts visible at left)
    let square_id = scene.add_tattva(
        Square::new(1.0, Vec4::new(0.96, 0.42, 0.28, 1.0)),
        Vec3::new(-4.0, 0.0, 0.0),
    );

    // The Target (hidden at right)
    let circle_id = scene.add_tattva(
        Circle::new(0.5, 64, Vec4::new(0.26, 0.70, 0.44, 1.0)),
        Vec3::new(-4.0, 0.0, 0.0), // Start at the same position as square
    );

    // Hide target initially
    if let Some(t) = scene.get_tattva_any_mut(circle_id) {
        let mut props = t.props().write();
        props.visible = false;
        props.opacity = 0.0;
    }

    let mut timeline = Timeline::new();

    // 1. Morph AND Move together
    // Note: We animate the 'target' (circle), morphing it from the 'source' (square).
    // The source (square) will be automatically hidden at 1.0s.

    // Morph
    timeline
        .animate(circle_id)
        .at(1.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .morph_from(square_id)
        .spawn();

    // Simultaneously MOVE the target (which is now our morphing shape)
    // We animate it from its current position (which we'll set to the source's start)
    // to the target position.
    timeline
        .animate(circle_id)
        .at(1.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(4.0, 0.0, 0.0))
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    println!("Running Morph + Translation Showcase...");
    println!("1.0s: Shape will start moving from left to right while morphing.");

    App::new()?.with_scene(scene).run_app()
}
