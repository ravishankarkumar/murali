use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::path::Path;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::primitives::square::Square;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // --- Row 1: fill only (no outline) ---

    // Circle, fill only
    let circle_id = scene.add_tattva(
        Circle::new(1.0, 48, GREEN_D),
        Vec3::new(-5.5, 2.5, 0.0),
    );

    // Square, fill only
    let square_id = scene.add_tattva(
        Square::new(1.8, RED_B),
        Vec3::new(-1.8, 2.5, 0.0),
    );

    // Rectangle, fill only
    let rect_id = scene.add_tattva(
        Rectangle::new(2.4, 1.4, BLUE_D),
        Vec3::new(2.5, 2.5, 0.0),
    );

    // --- Row 2: stroke outline ---

    // Circle with stroke
    let circle_stroke_id = scene.add_tattva(
        Circle::new(1.0, 48, Vec4::new(0.0, 0.0, 0.0, 0.0)) // transparent fill
            .with_stroke(0.06, GREEN_D),
        Vec3::new(-5.5, -0.5, 0.0),
    );

    // Square with stroke
    let square_stroke_id = scene.add_tattva(
        Square::new(1.8, Vec4::new(0.0, 0.0, 0.0, 0.0))
            .with_stroke(0.06, RED_B),
        Vec3::new(-1.8, -0.5, 0.0),
    );

    // Rectangle with stroke
    let rect_stroke_id = scene.add_tattva(
        Rectangle::new(2.4, 1.4, Vec4::new(0.0, 0.0, 0.0, 0.0))
            .with_stroke(0.06, BLUE_D),
        Vec3::new(2.5, -0.5, 0.0),
    );

    // --- Row 3: fill + stroke ---

    let circle_both_id = scene.add_tattva(
        Circle::new(1.0, 48, Vec4::new(GREEN_D.x, GREEN_D.y, GREEN_D.z, 0.35))
            .with_stroke(0.06, GREEN_D),
        Vec3::new(-5.5, -3.5, 0.0),
    );

    let square_both_id = scene.add_tattva(
        Square::new(1.8, Vec4::new(RED_B.x, RED_B.y, RED_B.z, 0.35))
            .with_stroke(0.06, RED_B),
        Vec3::new(-1.8, -3.5, 0.0),
    );

    // Custom path — always works directly
    let star_id = scene.add_tattva(
        Path::new()
            .move_to(vec2(0.0, 1.2))
            .line_to(vec2(0.3, 0.4))
            .line_to(vec2(1.2, 0.4))
            .line_to(vec2(0.5, -0.1))
            .line_to(vec2(0.7, -0.9))
            .line_to(vec2(0.0, -0.4))
            .line_to(vec2(-0.7, -0.9))
            .line_to(vec2(-0.5, -0.1))
            .line_to(vec2(-1.2, 0.4))
            .line_to(vec2(-0.3, 0.4))
            .close()
            .with_color(YELLOW_B),
        Vec3::new(2.5, -3.5, 0.0),
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // Draw row 1
    timeline
        .animate(circle_id)
        .at(0.0)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(square_id)
        .at(0.3)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(rect_id)
        .at(0.6)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();

    // Draw row 2
    timeline
        .animate(circle_stroke_id)
        .at(0.5)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(square_stroke_id)
        .at(0.8)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(rect_stroke_id)
        .at(1.1)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();

    // Draw row 3
    timeline
        .animate(circle_both_id)
        .at(1.0)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(square_both_id)
        .at(1.3)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();
    timeline
        .animate(star_id)
        .at(1.6)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .draw()
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
