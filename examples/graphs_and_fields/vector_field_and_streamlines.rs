/// Vector Field and StreamLines Combined
/// Shows both representations of the same vector field side by side
use glam::{Vec2, Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::graph::stream_lines::{StreamLines, line_start_points};
use murali::frontend::collection::graph::vector_field::VectorField;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Vector Field vs StreamLines", 0.36)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.add_tattva(
        Label::new("Same field, different visualizations", 0.14)
            .with_color(GRAY_B),
        Vec3::new(0.0, 3.2, 0.0),
    );

    // Define a common vector field function (vortex with radial component)
    let field_fn = |pos: Vec2| {
        let r = pos.length();
        if r < 0.1 {
            Vec2::ZERO
        } else {
            // Spiral: circular + outward
            Vec2::new(-pos.y + pos.x * 0.2, pos.x + pos.y * 0.2)
        }
    };

    // Left side: Vector Field
    scene.add_tattva(
        Label::new("Vector Field", 0.18).with_color(GRAY_B),
        Vec3::new(-4.0, 2.5, 0.0),
    );

    let vector_field = VectorField::new((-3.0, 3.0), (-2.0, 2.0), 13, 9, field_fn)
        .with_color(BLUE_B)
        .with_length_scale(0.4)
        .with_arrow_style(0.025, 0.08, 0.06);

    scene.add_tattva(vector_field, 4.0 * LEFT);

    // Right side: StreamLines
    scene.add_tattva(
        Label::new("StreamLines", 0.18).with_color(GRAY_B),
        Vec3::new(4.0, 2.5, 0.0),
    );

    // Create starting points in a grid pattern
    let mut start_points = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            let x = -2.5 + i as f32 * 1.2;
            let y = -1.5 + j as f32 * 0.8;
            start_points.push(Vec2::new(x, y));
        }
    }

    let stream_lines = StreamLines::new(start_points, field_fn)
        .with_color(RED_B)
        .with_thickness(0.025)
        .with_step_size(0.06)
        .with_max_steps(150)
        .with_bounds(Vec2::new(-3.0, -2.0), Vec2::new(3.0, 2.0));

    scene.add_tattva(stream_lines, 4.0 * RIGHT);

    // Bottom: Combined view
    scene.add_tattva(
        Label::new("Combined View", 0.18).with_color(GRAY_B),
        Vec3::new(0.0, -2.2, 0.0),
    );

    // Vector field (lighter)
    let combined_vectors = VectorField::new((-5.0, 5.0), (-3.5, -0.5), 17, 7, field_fn)
        .with_color(BLUE_B)
        .with_length_scale(0.3)
        .with_arrow_style(0.02, 0.06, 0.05);

    scene.add_tattva(combined_vectors, ORIGIN);

    // StreamLines on top
    let combined_start_points = line_start_points(Vec2::new(-4.5, -0.5), Vec2::new(4.5, -0.5), 9);
    let combined_streams = StreamLines::new(combined_start_points, field_fn)
        .with_color(RED_B)
        .with_thickness(0.03)
        .with_step_size(0.06)
        .with_max_steps(150)
        .with_bounds(Vec2::new(-5.0, -3.5), Vec2::new(5.0, -0.5));

    scene.add_tattva(combined_streams, ORIGIN);

    // Add explanation
    scene.add_tattva(
        Label::new(
            "Arrows show direction at points | Lines show flow paths",
            0.12,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -3.8, 0.0),
    );

    App::new()?.with_scene(scene).run_app()
}
