use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::path::Path;
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

    // Create a circle and convert it to a path for write effect
    let circle = Circle::new(1.5, 48, Vec4::new(0.19, 0.64, 0.33, 1.0));
    let circle_path = circle.to_path();
    let circle_id = scene.add_tattva(circle_path, Vec3::new(-3.0, 0.0, 0.0));

    // Create a filled circle with write effect
    let filled_circle = Circle::new(1.2, 48, Vec4::new(0.16, 0.50, 0.73, 1.0));
    let mut filled_circle_path = filled_circle.to_path();
    // Add fill to the circle
    filled_circle_path.style = Style::new()
        .with_stroke(filled_circle_path.style.stroke.unwrap_or_default())
        .with_fill(ColorSource::Solid(Vec4::new(0.16, 0.50, 0.73, 1.0)));
    let filled_circle_id = scene.add_tattva(filled_circle_path, Vec3::new(-3.0, -2.5, 0.0));

    // Create a rectangle and convert it to a path for write effect
    let rect = Rectangle::new(2.0, 1.5, Vec4::new(0.92, 0.26, 0.21, 1.0));
    let rect_path = rect.to_path();
    let rect_id = scene.add_tattva(rect_path, Vec3::new(0.0, 0.0, 0.0));

    // Create a custom path (star-like shape)
    let star_path = Path::new()
        .move_to(vec2(0.0, 1.5))
        .line_to(vec2(0.3, 0.5))
        .line_to(vec2(1.5, 0.5))
        .line_to(vec2(0.6, 0.0))
        .line_to(vec2(0.9, -1.0))
        .line_to(vec2(0.0, -0.5))
        .line_to(vec2(-0.9, -1.0))
        .line_to(vec2(-0.6, 0.0))
        .line_to(vec2(-1.5, 0.5))
        .line_to(vec2(-0.3, 0.5))
        .close()
        .with_color(Vec4::new(0.96, 0.80, 0.19, 1.0));
    let star_id = scene.add_tattva(star_path, Vec3::new(3.0, 0.0, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    // Animate the circle with write effect
    timeline.animate(circle_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    // Animate the filled circle with write effect
    timeline.animate(filled_circle_id)
        .at(0.5)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    // Animate the rectangle with write effect (delayed)
    timeline.animate(rect_id)
        .at(2.5)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    // Animate the star with write effect (delayed more)
    timeline.animate(star_id)
        .at(5.0)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .write()
        .spawn();

    // Unwrite the circle (reverse effect)
    timeline.animate(circle_id)
        .at(8.0)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    // Unwrite the filled circle
    timeline.animate(filled_circle_id)
        .at(8.5)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    // Unwrite the rectangle
    timeline.animate(rect_id)
        .at(10.5)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    // Unwrite the star
    timeline.animate(star_id)
        .at(13.0)
        .for_duration(2.0)
        .ease(Ease::InCubic)
        .unwrite()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
