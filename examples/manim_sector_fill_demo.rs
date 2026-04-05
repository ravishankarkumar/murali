/// Demonstration of Manim-style sector filling with write effects
/// Shows how filled shapes progressively fill as their outlines are drawn
use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::primitives::to_path::ToPath;
use murali::frontend::animation::Ease;
use murali::engine::timeline::Timeline;
use murali::frontend::style::Style;
use murali::projection::style::ColorSource;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // ===== FILLED CIRCLE (Blue) =====
    let circle = Circle::new(1.2, 64, Vec4::new(0.16, 0.50, 0.73, 1.0));
    let mut circle_path = circle.to_path();
    circle_path.style = Style::new()
        .with_stroke(circle_path.style.stroke.unwrap_or_default())
        .with_fill(ColorSource::Solid(Vec4::new(0.16, 0.50, 0.73, 1.0)));
    let circle_id = scene.add_tattva(circle_path, Vec3::new(-3.0, 1.5, 0.0));

    // ===== FILLED RECTANGLE (Red) =====
    let rect = Rectangle::new(2.0, 1.5, Vec4::new(0.92, 0.26, 0.21, 1.0));
    let mut rect_path = rect.to_path();
    rect_path.style = Style::new()
        .with_stroke(rect_path.style.stroke.unwrap_or_default())
        .with_fill(ColorSource::Solid(Vec4::new(0.92, 0.26, 0.21, 1.0)));
    let rect_id = scene.add_tattva(rect_path, Vec3::new(0.0, 1.5, 0.0));

    // ===== FILLED CIRCLE (Green) =====
    let circle2 = Circle::new(1.2, 64, Vec4::new(0.19, 0.64, 0.33, 1.0));
    let mut circle2_path = circle2.to_path();
    circle2_path.style = Style::new()
        .with_stroke(circle2_path.style.stroke.unwrap_or_default())
        .with_fill(ColorSource::Solid(Vec4::new(0.19, 0.64, 0.33, 1.0)));
    let circle2_id = scene.add_tattva(circle2_path, Vec3::new(3.0, 1.5, 0.0));

    // ===== OUTLINE CIRCLE (Yellow, no fill) =====
    let circle3 = Circle::new(1.2, 64, Vec4::new(0.96, 0.80, 0.19, 1.0));
    let circle3_path = circle3.to_path();
    let circle3_id = scene.add_tattva(circle3_path, Vec3::new(-3.0, -1.5, 0.0));

    // ===== OUTLINE RECTANGLE (Purple, no fill) =====
    let rect2 = Rectangle::new(2.0, 1.5, Vec4::new(0.61, 0.35, 0.71, 1.0));
    let rect2_path = rect2.to_path();
    let rect2_id = scene.add_tattva(rect2_path, Vec3::new(0.0, -1.5, 0.0));

    // ===== OUTLINE CIRCLE (Cyan, no fill) =====
    let circle4 = Circle::new(1.2, 64, Vec4::new(0.20, 0.80, 0.80, 1.0));
    let circle4_path = circle4.to_path();
    let circle4_id = scene.add_tattva(circle4_path, Vec3::new(3.0, -1.5, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    // ===== ANIMATIONS =====
    // Top row: Filled shapes with sector filling
    timeline.animate(circle_id)
        .at(0.0)
        .for_duration(2.5)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    timeline.animate(rect_id)
        .at(0.3)
        .for_duration(2.5)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    timeline.animate(circle2_id)
        .at(0.6)
        .for_duration(2.5)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    // Bottom row: Outline-only shapes (no fill)
    timeline.animate(circle3_id)
        .at(3.5)
        .for_duration(2.5)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    timeline.animate(rect2_id)
        .at(3.8)
        .for_duration(2.5)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    timeline.animate(circle4_id)
        .at(4.1)
        .for_duration(2.5)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    // Unwrite animations
    timeline.animate(circle_id)
        .at(7.0)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    timeline.animate(rect_id)
        .at(7.3)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    timeline.animate(circle2_id)
        .at(7.6)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    timeline.animate(circle3_id)
        .at(10.0)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    timeline.animate(rect2_id)
        .at(10.3)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    timeline.animate(circle4_id)
        .at(10.6)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
