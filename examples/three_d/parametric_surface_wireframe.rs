/// Parametric Surface with Wireframe and Heat Map Coloring
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::axes3d::Axes3D;
use murali::frontend::collection::graph::parametric_surface::{
    ParametricSurface, SurfaceRenderMode,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("3D Surfaces with Wireframe & Heat Map", 0.36)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Left: Wavy surface with wireframe
    scene.add_tattva(
        Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-1.0, 1.0))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        Vec3::new(-4.0, 0.5, 0.0),
    );

    let wavy_wireframe = ParametricSurface::new((-PI, PI), (-PI, PI), |u, v| {
        let x = u;
        let y = v;
        let z = (u * 0.8).sin() * (v * 0.8).cos();
        Vec3::new(x, y, z)
    })
    .with_samples(24, 24)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_color_fn(|z| {
        // Heat map: blue (cold) to red (hot)
        let normalized = (z + 1.0) / 2.0; // Map from [-1, 1] to [0, 1]
        if normalized < 0.5 {
            // Blue to cyan
            let t = normalized * 2.0;
            Vec4::new(0.0, t, 1.0, 1.0)
        } else {
            // Cyan to red
            let t = (normalized - 0.5) * 2.0;
            Vec4::new(t, 1.0 - t, 0.0, 1.0)
        }
    });

    scene.add_tattva(wavy_wireframe, Vec3::new(-4.0, 0.5, 0.0));

    scene.add_tattva(
        Label::new("Wireframe Heat Map", 0.16).with_color(GRAY_B),
        Vec3::new(-4.0, -2.2, 0.0),
    );

    // Right: Sphere with solid + wireframe
    scene.add_tattva(
        Axes3D::new((-1.5, 1.5), (-1.5, 1.5), (-1.5, 1.5))
            .with_step(0.5)
            .with_axis_thickness(0.03),
        Vec3::new(4.0, 0.5, 0.0),
    );

    let sphere_solid_wire = ParametricSurface::new((0.0, PI), (0.0, 2.0 * PI), |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
    })
    .with_samples(32, 32)
    .with_render_mode(SurfaceRenderMode::SolidWithWireframe)
    .with_color(Vec4::new(TEAL_C.x, TEAL_C.y, TEAL_C.z, 0.6))
    .with_color_fn(|z| {
        // Gradient from purple to yellow based on height
        let normalized = (z + 1.0) / 2.0;
        Vec4::new(
            0.5 + 0.5 * normalized,
            0.2 + 0.8 * normalized,
            0.8 - 0.8 * normalized,
            1.0,
        )
    });

    scene.add_tattva(sphere_solid_wire, Vec3::new(4.0, 0.5, 0.0));

    scene.add_tattva(
        Label::new("Solid + Wireframe", 0.16).with_color(GRAY_B),
        Vec3::new(4.0, -2.2, 0.0),
    );

    // Bottom: Torus with wireframe
    scene.add_tattva(
        Axes3D::new((-2.5, 2.5), (-2.5, 2.5), (-1.0, 1.0))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        3.5 * DOWN,
    );

    let torus_wireframe = ParametricSurface::new((0.0, 2.0 * PI), (0.0, 2.0 * PI), |u, v| {
        let r = 0.4;
        let major_radius = 1.2;
        let x = (major_radius + r * v.cos()) * u.cos();
        let y = (major_radius + r * v.cos()) * u.sin();
        let z = r * v.sin();
        Vec3::new(x, y, z)
    })
    .with_samples(32, 24)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_color_fn(|z| {
        // Rainbow gradient
        let normalized = (z + 0.5) / 1.0; // Map from [-0.5, 0.5] to [0, 1]
        let hue = normalized * 6.0;
        match hue as i32 {
            0 => Vec4::new(1.0, normalized * 6.0, 0.0, 1.0), // Red to yellow
            1 => Vec4::new(1.0 - (normalized - 1.0 / 6.0) * 6.0, 1.0, 0.0, 1.0), // Yellow to green
            2 => Vec4::new(0.0, 1.0, (normalized - 2.0 / 6.0) * 6.0, 1.0), // Green to cyan
            3 => Vec4::new(0.0, 1.0 - (normalized - 3.0 / 6.0) * 6.0, 1.0, 1.0), // Cyan to blue
            4 => Vec4::new((normalized - 4.0 / 6.0) * 6.0, 0.0, 1.0, 1.0), // Blue to magenta
            _ => Vec4::new(1.0, 0.0, 1.0 - (normalized - 5.0 / 6.0) * 6.0, 1.0), // Magenta to red
        }
    });

    scene.add_tattva(torus_wireframe, 3.5 * DOWN);

    scene.add_tattva(
        Label::new("Torus Rainbow Wireframe", 0.16).with_color(GRAY_B),
        Vec3::new(0.0, -5.7, 0.0),
    );

    App::new()?.with_scene(scene).run_app()
}
