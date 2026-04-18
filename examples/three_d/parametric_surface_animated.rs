/// Parametric Surface with Animation
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::composite::axes3d::Axes3D;
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Animated 3D Surface", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // 3D Axes
    scene.add_tattva(
        Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0))
            .with_step(1.0)
            .with_axis_thickness(0.04),
        ORIGIN,
    );

    // Parametric surface: Möbius-like strip (starts invisible)
    let surface = ParametricSurface::new(
        (0.0, 2.0 * PI), // u_range
        (-0.5, 0.5),     // v_range
        |u, v| {
            let r = 1.0 + v * (u * 0.5).cos();
            let x = r * u.cos();
            let y = r * u.sin();
            let z = v * (u * 0.5).sin();
            Vec3::new(x, y, z)
        },
    )
    .with_samples(48, 16)
    .with_color(GOLD_C)
    .with_write_progress(0.0); // Start invisible

    let surface_id = scene.add_tattva(surface, ORIGIN);

    // Subtitle
    scene.add_tattva(
        Label::new("Möbius-like Parametric Surface", 0.18)
            .with_color(GRAY_B),
        Vec3::new(0.0, -2.2, 0.0),
    );

    // Animation: write the surface, then rotate it
    let mut timeline = Timeline::new();

    // Phase 1: Write the surface (0.0 to 3.0 seconds)
    timeline
        .animate(surface_id)
        .at(0.0)
        .for_duration(3.0)
        .ease(Ease::InOutQuad)
        .write_surface()
        .spawn();

    // Phase 2: Rotate the surface while it's visible (3.0 to 8.0 seconds)
    timeline
        .animate(surface_id)
        .at(3.0)
        .for_duration(5.0)
        .ease(Ease::Linear)
        .rotate_to(glam::Quat::from_axis_angle(Vec3::Z, 2.0 * PI))
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
