use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // The Source (starts visible at left)
    let square_id = scene.add_tattva(
        Square::new(1.0, ORANGE_D),
        4.0 * LEFT,
    );

    // The Target (hidden at right)
    let circle_id = scene.add_tattva(
        Circle::new(0.5, 64, GREEN_C),
        4.0 * LEFT, // Start at the same position as square
    );

    scene.hide_tattva(circle_id);

    let mut timeline = Timeline::new();

    // 1. Morph AND Move together
    // Note: We animate the 'target' (circle), morphing it from the 'source' (square).
    // The source (square) will be automatically hidden at 1.0s.

    // Morph
    timeline
        .animate(circle_id)
        .at(1.0)
        .for_duration(2.0)
        .ease(Ease::InOutCubic)
        .morph_from(square_id)
        .spawn();

    // Simultaneously MOVE the target (which is now our morphing shape)
    // We animate it from its current position (which we'll set to the source's start)
    // to the target position.
    timeline
        .animate(circle_id)
        .at(1.0)
        .for_duration(2.0)
        .ease(Ease::InOutCubic)
        .move_to(4.0 * RIGHT)
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
