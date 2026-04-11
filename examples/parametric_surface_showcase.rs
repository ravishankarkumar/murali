/// Parametric Surface Showcase - 3D surfaces like Manim
use glam::{Vec3, Vec4};
use murali::App;
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
        Label::new("3D Parametric Surfaces", 0.4)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // 3D Axes
    scene.add_tattva(
        Axes3D::new((-1.5, 1.5), (-1.5, 1.5), (-1.5, 1.5))
            .with_step(0.5)
            .with_axis_thickness(0.04),
        Vec3::new(0.0, 0.0, 0.0),
    );

    // Sphere surface
    // Parametric form: (sin(u)*cos(v), sin(u)*sin(v), cos(u))
    let sphere = ParametricSurface::new(
        (0.0, PI),           // u_range (theta: 0 to π)
        (0.0, 2.0 * PI),     // v_range (phi: 0 to 2π)
        |u, v| {
            let sin_u = u.sin();
            Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
        },
    )
    .with_samples(40, 40)
    .with_color(Vec4::new(0.44, 0.84, 0.71, 1.0));

    scene.add_tattva(sphere, Vec3::new(0.0, 0.0, 0.0));

    // Subtitle
    scene.add_tattva(
        Label::new("Sphere: sin(u)cos(v), sin(u)sin(v), cos(u)", 0.18)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -1.8, 0.0),
    );

    App::new()?.with_scene(scene).run_app()
}
