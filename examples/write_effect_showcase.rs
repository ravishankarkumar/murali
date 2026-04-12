use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::square::Square;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::primitives::path::Path;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // --- Row 1: fill only (no outline) ---

    // Circle, fill only
    let circle_id = scene.add_tattva(
        Circle::new(1.0, 48, Vec4::new(0.19, 0.64, 0.33, 1.0)),
        Vec3::new(-5.5, 2.5, 0.0),
    );


    // Square, fill only
    let square_id = scene.add_tattva(
        Square::new(1.8, Vec4::new(0.92, 0.26, 0.21, 1.0)),
        Vec3::new(-1.8, 2.5, 0.0),
    );


    // Rectangle, fill only
    let rect_id = scene.add_tattva(
        Rectangle::new(2.4, 1.4, Vec4::new(0.22, 0.50, 0.96, 1.0)),
        Vec3::new(2.5, 2.5, 0.0),
    );


    // --- Row 2: stroke outline ---

    // Circle with stroke
    let circle_stroke_id = scene.add_tattva(
        Circle::new(1.0, 48, Vec4::new(0.0, 0.0, 0.0, 0.0)) // transparent fill
            .with_stroke(0.06, Vec4::new(0.19, 0.64, 0.33, 1.0)),
        Vec3::new(-5.5, -0.5, 0.0),
    );


    // Square with stroke
    let square_stroke_id = scene.add_tattva(
        Square::new(1.8, Vec4::new(0.0, 0.0, 0.0, 0.0))
            .with_stroke(0.06, Vec4::new(0.92, 0.26, 0.21, 1.0)),
        Vec3::new(-1.8, -0.5, 0.0),
    );


    // Rectangle with stroke
    let rect_stroke_id = scene.add_tattva(
        Rectangle::new(2.4, 1.4, Vec4::new(0.0, 0.0, 0.0, 0.0))
            .with_stroke(0.06, Vec4::new(0.22, 0.50, 0.96, 1.0)),
        Vec3::new(2.5, -0.5, 0.0),
    );


    // --- Row 3: fill + stroke ---

    let circle_both_id = scene.add_tattva(
        Circle::new(1.0, 48, Vec4::new(0.19, 0.64, 0.33, 0.35))
            .with_stroke(0.06, Vec4::new(0.19, 0.64, 0.33, 1.0)),
        Vec3::new(-5.5, -3.5, 0.0),
    );


    let square_both_id = scene.add_tattva(
        Square::new(1.8, Vec4::new(0.92, 0.26, 0.21, 0.35))
            .with_stroke(0.06, Vec4::new(0.92, 0.26, 0.21, 1.0)),
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
            .with_color(Vec4::new(0.96, 0.80, 0.19, 1.0)),
        Vec3::new(2.5, -3.5, 0.0),
    );


    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    // Write row 1
    timeline.animate(circle_id).at(0.0).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(square_id).at(0.3).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(rect_id).at(0.6).for_duration(1.5).ease(Ease::OutCubic).write().spawn();

    // Write row 2
    timeline.animate(circle_stroke_id).at(0.5).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(square_stroke_id).at(0.8).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(rect_stroke_id).at(1.1).for_duration(1.5).ease(Ease::OutCubic).write().spawn();

    // Write row 3
    timeline.animate(circle_both_id).at(1.0).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(square_both_id).at(1.3).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(star_id).at(1.6).for_duration(1.5).ease(Ease::OutCubic).write().spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
