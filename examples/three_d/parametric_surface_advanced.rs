/// Advanced Parametric Surfaces - Multiple 3D surfaces
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::axes3d::Axes3D;
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Advanced 3D Parametric Surfaces", 0.36)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Left: Torus
    scene.add_tattva(
        Axes3D::new((-3.0, 3.0), (-3.0, 3.0), (-1.5, 1.5))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        Vec3::new(-4.0, 0.5, 0.0),
    );

    // Torus: (R + r*cos(v))*cos(u), (R + r*cos(v))*sin(u), r*sin(v)
    // R = major radius, r = minor radius
    let torus = ParametricSurface::new(
        (0.0, 2.0 * PI), // u_range
        (0.0, 2.0 * PI), // v_range
        |u, v| {
            let r = 0.4; // minor radius
            let major_radius = 1.2;
            let x = (major_radius + r * v.cos()) * u.cos();
            let y = (major_radius + r * v.cos()) * u.sin();
            let z = r * v.sin();
            Vec3::new(x, y, z)
        },
    )
    .with_samples(32, 24)
    .with_color(GOLD_C);

    scene.add_tattva(torus, Vec3::new(-4.0, 0.5, 0.0));

    scene.add_tattva(
        Label::new("Torus", 0.16).with_color(GRAY_B),
        Vec3::new(-4.0, -2.2, 0.0),
    );

    // Right: Wavy surface
    scene.add_tattva(
        Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-1.0, 1.0))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        Vec3::new(4.0, 0.5, 0.0),
    );

    // Wavy surface: z = sin(x) * cos(y)
    let wavy = ParametricSurface::new(
        (-PI, PI), // u_range (x)
        (-PI, PI), // v_range (y)
        |u, v| {
            let x = u;
            let y = v;
            let z = (u * 0.8).sin() * (v * 0.8).cos();
            Vec3::new(x, y, z)
        },
    )
    .with_samples(36, 36)
    .with_color(TEAL_C);

    scene.add_tattva(wavy, Vec3::new(4.0, 0.5, 0.0));

    scene.add_tattva(
        Label::new("Wavy Surface", 0.16).with_color(GRAY_B),
        Vec3::new(4.0, -2.2, 0.0),
    );

    // Bottom: Klein bottle-like surface
    scene.add_tattva(
        Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-2.0, 2.0))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        3.5 * DOWN,
    );

    // Parametric surface with interesting topology
    let interesting = ParametricSurface::new(
        (0.0, 2.0 * PI), // u_range
        (0.0, PI),       // v_range
        |u, v| {
            let r = 1.0 + 0.5 * (u * 0.5).cos();
            let x = r * v.sin() * u.cos();
            let y = r * v.sin() * u.sin();
            let z = (u * 0.5).sin() + 0.3 * v.cos();
            Vec3::new(x, y, z)
        },
    )
    .with_samples(40, 28)
    .with_color(GRAY_A);

    scene.add_tattva(interesting, 3.5 * DOWN);

    scene.add_tattva(
        Label::new("Parametric Surface", 0.16).with_color(GRAY_B),
        Vec3::new(0.0, -5.7, 0.0),
    );

    App::new()?.with_scene(scene).run_app()
}
