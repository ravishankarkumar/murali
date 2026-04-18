use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::path::Path;
use murali::frontend::collection::primitives::square::Square;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // Vibrant color palette
    let purple = PURPLE_B;
    let cyan = TEAL_C;
    let yellow = YELLOW_D;
    let pink = PINK_B;

    // 1. A complex central "Diamond" path
    let diamond_id = scene.add_tattva(
        Path::new()
            .move_to(vec2(0.0, 2.0))
            .line_to(vec2(1.5, 0.0))
            .line_to(vec2(0.0, -2.0))
            .line_to(vec2(-1.5, 0.0))
            .close()
            .with_color(cyan)
            .with_thickness(0.08),
        ORIGIN,
    );

    // 2. Surrounding orbiting shapes
    let c1 = scene.add_tattva(
        Circle::new(0.5, 48, purple.lerp(Vec4::new(0.0, 0.0, 0.0, 0.0), 0.7))
            .with_stroke(0.05, purple),
        Vec3::new(-3.0, 1.5, 0.0),
    );

    let s1 = scene.add_tattva(
        Square::new(0.8, pink.lerp(Vec4::new(0.0, 0.0, 0.0, 0.0), 0.7)).with_stroke(0.05, pink),
        Vec3::new(3.0, 1.5, 0.0),
    );

    let c2 = scene.add_tattva(
        Circle::new(0.5, 48, yellow.lerp(Vec4::new(0.0, 0.0, 0.0, 0.0), 0.7))
            .with_stroke(0.05, yellow),
        Vec3::new(-3.0, -1.5, 0.0),
    );

    let s2 = scene.add_tattva(
        Square::new(0.8, yellow.lerp(Vec4::new(0.0, 0.0, 0.0, 0.0), 0.7)).with_stroke(0.05, yellow),
        Vec3::new(3.0, -1.5, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 8.0);

    // --- Animation Sequence ---

    // Phase 1: Draw everything in
    timeline
        .animate(diamond_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::InOutCubic)
        .draw()
        .spawn();

    timeline
        .animate(c1)
        .at(0.5)
        .for_duration(1.0)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(s1)
        .at(0.7)
        .for_duration(1.0)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(c2)
        .at(0.9)
        .for_duration(1.0)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(s2)
        .at(1.1)
        .for_duration(1.0)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();

    // Phase 2: A brief pause at full visibility
    let transition_start = 3.0;

    // Phase 3: Undraw everything out
    // Central diamond undraws first
    timeline
        .animate(diamond_id)
        .at(transition_start)
        .for_duration(1.5)
        .ease(Ease::InOutQuad)
        .undraw()
        .spawn();

    // Then surrounding shapes undraw in a cascade
    timeline
        .animate(c1)
        .at(transition_start + 0.3)
        .for_duration(1.0)
        .ease(Ease::InCubic)
        .undraw()
        .spawn();
    timeline
        .animate(s1)
        .at(transition_start + 0.5)
        .for_duration(1.0)
        .ease(Ease::InCubic)
        .undraw()
        .spawn();
    timeline
        .animate(c2)
        .at(transition_start + 0.7)
        .for_duration(1.0)
        .ease(Ease::InCubic)
        .undraw()
        .spawn();
    timeline
        .animate(s2)
        .at(transition_start + 0.9)
        .for_duration(1.0)
        .ease(Ease::InCubic)
        .undraw()
        .spawn();

    // Phase 4: Draw them back in but at different positions (optional, for flare)
    timeline
        .animate(diamond_id)
        .at(transition_start + 3.0)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
