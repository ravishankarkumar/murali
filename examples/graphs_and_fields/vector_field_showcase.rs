/// Vector Field Showcase
/// Demonstrates various vector field visualizations
use glam::{Vec2, Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::graph::vector_field::VectorField;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Vector Field Showcase", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // 1. Radial field (pointing outward from origin)
    scene.add_tattva(
        Label::new("Radial Field", 0.16).with_color(GRAY_B),
        4.5 * LEFT + 2.8 * UP,
    );

    let radial_field = VectorField::new(
        (-2.0, 2.0),     // x range
        (0.5, 2.5),      // y range
        9,               // x steps
        5,               // y steps
        |pos: Vec2| pos, // Vector points away from origin
    )
    .with_color(RED_B)
    .with_length_scale(0.3);

    scene.add_tattva(radial_field, 4.5 * LEFT);

    // 2. Circular/Rotational field
    scene.add_tattva(
        Label::new("Rotational Field", 0.16).with_color(GRAY_B),
        2.8 * UP,
    );

    let circular_field = VectorField::new(
        (-2.0, 2.0),
        (0.5, 2.5),
        9,
        5,
        |pos: Vec2| Vec2::new(-pos.y, pos.x), // Perpendicular to radial
    )
    .with_color(GREEN_B)
    .with_length_scale(0.3);

    scene.add_tattva(circular_field, ORIGIN);

    // 3. Gradient field (pointing right and up)
    scene.add_tattva(
        Label::new("Gradient Field", 0.16).with_color(GRAY_B),
        4.5 * RIGHT + 2.8 * UP,
    );

    let gradient_field = VectorField::new(
        (-2.0, 2.0),
        (0.5, 2.5),
        9,
        5,
        |pos: Vec2| Vec2::new(1.0, pos.y * 0.5), // Gradient in y direction
    )
    .with_color(BLUE_B)
    .with_length_scale(0.3);

    scene.add_tattva(gradient_field, 4.5 * RIGHT);

    // 4. Saddle point field
    scene.add_tattva(
        Label::new("Saddle Point", 0.16).with_color(GRAY_B),
        4.5 * LEFT + 0.8 * DOWN,
    );

    let saddle_field = VectorField::new(
        (-2.0, 2.0),
        (-2.5, -0.5),
        9,
        5,
        |pos: Vec2| Vec2::new(pos.x, -pos.y), // Saddle point at origin
    )
    .with_color(GOLD_C)
    .with_length_scale(0.3);

    scene.add_tattva(saddle_field, 4.5 * LEFT);

    // 5. Sine wave field
    scene.add_tattva(
        Label::new("Sine Wave Field", 0.16).with_color(GRAY_B),
        0.8 * DOWN,
    );

    let sine_field = VectorField::new((-2.0, 2.0), (-2.5, -0.5), 9, 5, |pos: Vec2| {
        Vec2::new((pos.y * PI).sin(), (pos.x * PI).cos())
    })
    .with_color(PURPLE_B)
    .with_length_scale(0.3);

    scene.add_tattva(sine_field, ORIGIN);

    // 6. Magnitude-colored field
    scene.add_tattva(
        Label::new("Magnitude Colored", 0.16).with_color(GRAY_B),
        4.5 * RIGHT + 0.8 * DOWN,
    );

    let colored_field = VectorField::new((-2.0, 2.0), (-2.5, -0.5), 9, 5, |pos: Vec2| pos)
        .with_color_fn(|magnitude: f32| {
            // Color from blue (low) to red (high)
            let t = (magnitude * 0.5).min(1.0);
            Vec4::new(t, 0.3, 1.0 - t, 0.9)
        })
        .with_length_scale(0.3);

    scene.add_tattva(colored_field, 4.5 * RIGHT);

    App::new()?.with_scene(scene).run_app()
}
