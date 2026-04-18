/// StreamLines Showcase
/// Demonstrates streamline visualization for various vector fields
use glam::{Vec2, Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::graph::stream_lines::{
    StreamLines, circle_start_points, line_start_points,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("StreamLines Showcase", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // 1. Radial flow (source)
    scene.add_tattva(
        Label::new("Radial Source", 0.16).with_color(GRAY_B),
        4.5 * LEFT + 2.8 * UP,
    );

    let radial_starts = circle_start_points(Vec2::new(0.0, 0.0), 0.3, 8);
    let radial_stream = StreamLines::new(
        radial_starts,
        |pos: Vec2| pos, // Radial outward
    )
    .with_color(Vec4::new(RED_B.x, RED_B.y, RED_B.z, 0.8))
    .with_thickness(0.025)
    .with_step_size(0.08)
    .with_max_steps(50)
    .with_bounds(Vec2::new(-2.0, 0.5), Vec2::new(2.0, 2.5));

    scene.add_tattva(radial_stream, 4.5 * LEFT);

    // 2. Circular/Vortex flow
    scene.add_tattva(
        Label::new("Vortex Flow", 0.16).with_color(GRAY_B),
        2.8 * UP,
    );

    let vortex_starts = circle_start_points(Vec2::new(0.0, 0.0), 1.5, 6);
    let vortex_stream = StreamLines::new(
        vortex_starts,
        |pos: Vec2| Vec2::new(-pos.y, pos.x), // Circular flow
    )
    .with_color(Vec4::new(GREEN_B.x, GREEN_B.y, GREEN_B.z, 0.8))
    .with_thickness(0.025)
    .with_step_size(0.08)
    .with_max_steps(100)
    .with_bounds(Vec2::new(-2.0, 0.5), Vec2::new(2.0, 2.5));

    scene.add_tattva(vortex_stream, ORIGIN);

    // 3. Saddle point flow
    scene.add_tattva(
        Label::new("Saddle Point", 0.16).with_color(GRAY_B),
        4.5 * RIGHT + 2.8 * UP,
    );

    let saddle_starts = line_start_points(Vec2::new(-1.5, 1.5), Vec2::new(1.5, 1.5), 5);
    let saddle_stream = StreamLines::new(
        saddle_starts,
        |pos: Vec2| Vec2::new(pos.x, -pos.y), // Saddle point
    )
    .with_color(Vec4::new(BLUE_B.x, BLUE_B.y, BLUE_B.z, 0.8))
    .with_thickness(0.025)
    .with_step_size(0.08)
    .with_max_steps(50)
    .with_bounds(Vec2::new(-2.0, 0.5), Vec2::new(2.0, 2.5));

    scene.add_tattva(saddle_stream, 4.5 * RIGHT);

    // 4. Uniform flow with obstacle (flow around)
    scene.add_tattva(
        Label::new("Flow Around Obstacle", 0.16).with_color(GRAY_B),
        4.5 * LEFT + 0.8 * DOWN,
    );

    let obstacle_starts = line_start_points(Vec2::new(-1.8, 0.5), Vec2::new(-1.8, 2.0), 6);
    let obstacle_stream = StreamLines::new(obstacle_starts, |pos: Vec2| {
        // Uniform flow to the right, but deflected by obstacle at origin
        let obstacle_pos = Vec2::ZERO;
        let to_obstacle = pos - obstacle_pos;
        let dist = to_obstacle.length();

        if dist < 0.1 {
            Vec2::ZERO
        } else {
            // Uniform flow + repulsion from obstacle
            let uniform = Vec2::new(1.0, 0.0);
            let repulsion = to_obstacle.normalize() * (0.3 / (dist * dist));
            uniform + repulsion
        }
    })
    .with_color(Vec4::new(GOLD_C.x, GOLD_C.y, GOLD_C.z, 0.8))
    .with_thickness(0.025)
    .with_step_size(0.06)
    .with_max_steps(80)
    .with_bounds(Vec2::new(-2.0, -2.5), Vec2::new(2.0, -0.5));

    scene.add_tattva(obstacle_stream, 4.5 * LEFT);

    // 5. Dipole field
    scene.add_tattva(
        Label::new("Dipole Field", 0.16).with_color(GRAY_B),
        0.8 * DOWN,
    );

    let dipole_starts = line_start_points(Vec2::new(-1.5, 2.0), Vec2::new(1.5, 2.0), 7);
    let dipole_stream = StreamLines::new(dipole_starts, |pos: Vec2| {
        // Positive charge at (-0.5, 0), negative at (0.5, 0)
        let pos_charge = Vec2::new(-0.5, 0.0);
        let neg_charge = Vec2::new(0.5, 0.0);

        let to_pos = pos - pos_charge;
        let to_neg = pos - neg_charge;

        let dist_pos = to_pos.length().max(0.2);
        let dist_neg = to_neg.length().max(0.2);

        let force_pos = to_pos.normalize() / (dist_pos * dist_pos);
        let force_neg = -to_neg.normalize() / (dist_neg * dist_neg);

        (force_pos + force_neg) * 0.5
    })
    .with_color(Vec4::new(PURPLE_B.x, PURPLE_B.y, PURPLE_B.z, 0.8))
    .with_thickness(0.025)
    .with_step_size(0.06)
    .with_max_steps(100)
    .with_bounds(Vec2::new(-2.0, -2.5), Vec2::new(2.0, -0.5));

    scene.add_tattva(dipole_stream, ORIGIN);

    // 6. Magnitude-colored streamlines
    scene.add_tattva(
        Label::new("Magnitude Colored", 0.16).with_color(GRAY_B),
        4.5 * RIGHT + 0.8 * DOWN,
    );

    let colored_starts = StreamLines::from_grid((-1.8, 1.8), (0.6, 2.4), 3, 4, |pos: Vec2| {
        // Spiral flow
        Vec2::new(-pos.y + pos.x * 0.3, pos.x + pos.y * 0.3)
    })
    .with_color_fn(|_pos: Vec2, magnitude: f32| {
        // Color from blue (slow) to red (fast)
        let t = (magnitude * 0.5).min(1.0);
        Vec4::new(t, 0.3, 1.0 - t, 0.8)
    })
    .with_thickness(0.025)
    .with_step_size(0.06)
    .with_max_steps(80)
    .with_bounds(Vec2::new(-2.0, -2.5), Vec2::new(2.0, -0.5));

    scene.add_tattva(colored_starts, 4.5 * RIGHT);

    App::new()?.with_scene(scene).run_app()
}
