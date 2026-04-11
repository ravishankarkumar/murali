/// TracedPath Example: Rolling Circle
/// Demonstrates a circle rolling along a surface while tracing the path of a point on its circumference
/// This creates a cycloid curve, similar to Manim's TracedPath animation
use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use murali::frontend::collection::utility::TracedPath;
use murali::frontend::style::{Style, ColorSource};
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Traced Path: Rolling Circle", 0.36)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Create a rolling circle with gradient (smaller for multiple cycles)
    let circle_radius = 0.3;
    let circle = Circle::new(circle_radius, 32, Vec4::new(0.2, 0.6, 0.9, 0.8))
        .with_style(
            Style::new().with_fill(
                ColorSource::LinearGradient {
                    start: vec2(-circle_radius, 0.0),
                    end: vec2(circle_radius, 0.0),
                    stops: vec![
                        (0.0, Vec4::new(0.1, 0.3, 0.8, 1.0)),    // Dark blue
                        (0.5, Vec4::new(0.3, 0.7, 1.0, 1.0)),    // Cyan
                        (1.0, Vec4::new(0.8, 0.9, 1.0, 1.0)),    // Light blue
                    ],
                },
            )
        )
        .with_stroke(0.03, Vec4::new(0.1, 0.4, 0.7, 1.0));

    let circle_id = scene.add_tattva(circle, Vec3::new(-6.0, -1.5, 0.0));

    // Create a traced path that tracks a point on the circle's circumference
    // The point starts at the bottom of the circle (the contact point with the ground)
    // As the circle rotates, this point traces a cycloid curve
    let traced_path = TracedPath::new(
        circle_id,
        move |circle_pos: Vec3, circle_rot: glam::Quat| {
            // Define a point on the circle's circumference in local space (bottom of circle)
            // This point is fixed relative to the circle and rotates with it
            let local_point = Vec3::new(0.0, -circle_radius, 0.0);
            
            // Apply the circle's rotation to get the point's position in world space
            let rotated_offset = circle_rot * local_point;
            
            // Add to the circle's center position to get the final world position
            circle_pos + rotated_offset
        },
        Vec4::new(1.0, 0.2, 0.2, 0.9),  // Red traced path
        0.06,  // Thickness (thicker for visibility)
    )
    .with_min_distance(0.001)  // Much smaller distance to capture more detail
    .with_max_points(10000);

    let traced_path_id = scene.add_tattva(traced_path, Vec3::ZERO);

    // Subtitle
    scene.add_tattva(
        Label::new("Cycloid Curve", 0.18)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.2, 0.0),
    );

    // Animation: Move the circle from left to right while rotating
    let mut timeline = Timeline::new();

    // For a rolling circle, the rotation angle equals distance / radius
    // This ensures the circle rolls without slipping
    let distance = 12.0;  // Total distance to travel (longer to show multiple cycles)
    let duration = 10.0;  // Duration in seconds
    let rotation_amount = distance / circle_radius;  // Radians to rotate (distance/radius)

    // Move the circle horizontally
    timeline
        .animate(circle_id)
        .at(0.0)
        .for_duration(duration)
        .ease(Ease::Linear)
        .move_to(Vec3::new(6.0, -1.5, 0.0))
        .spawn();

    // Rotate the circle (rolling motion)
    // Positive rotation for rolling to the right (counter-clockwise when viewed from front)
    timeline
        .animate(circle_id)
        .at(0.0)
        .for_duration(duration)
        .ease(Ease::Linear)
        .rotate_to(glam::Quat::from_axis_angle(Vec3::Z, rotation_amount))
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
