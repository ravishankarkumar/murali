/// Showcase: Multiple Animated Wireframe Surfaces with Heat Maps
/// Demonstrates progressive grid line drawing with color-based height mapping
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
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
        Label::new("Animated Wireframe Surfaces", 0.36)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Left: Wavy surface with heat map
    scene.add_tattva(
        Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-1.0, 1.0))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        Vec3::new(-4.0, 0.5, 0.0),
    );

    let wavy = ParametricSurface::new((-PI, PI), (-PI, PI), |u, v| {
        Vec3::new(u, v, (u * 0.8).sin() * (v * 0.8).cos())
    })
    .with_samples(24, 24)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_write_progress(0.0)
    .with_color_fn(|z| {
        let normalized = (z + 1.0) / 2.0;
        if normalized < 0.5 {
            let t = normalized * 2.0;
            Vec4::new(0.0, t, 1.0, 1.0)
        } else {
            let t = (normalized - 0.5) * 2.0;
            Vec4::new(t, 1.0 - t, 0.0, 1.0)
        }
    });

    let wavy_id = scene.add_tattva(wavy, Vec3::new(-4.0, 0.5, 0.0));

    scene.add_tattva(
        Label::new("Wavy Heat Map", 0.16).with_color(GRAY_B),
        Vec3::new(-4.0, -2.2, 0.0),
    );

    // Right: Sphere with rainbow gradient
    scene.add_tattva(
        Axes3D::new((-1.5, 1.5), (-1.5, 1.5), (-1.5, 1.5))
            .with_step(0.5)
            .with_axis_thickness(0.03),
        Vec3::new(4.0, 0.5, 0.0),
    );

    let sphere = ParametricSurface::new((0.0, PI), (0.0, 2.0 * PI), |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), sin_u * v.sin(), u.cos())
    })
    .with_samples(28, 28)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_write_progress(0.0)
    .with_color_fn(|z| {
        // Purple to yellow gradient
        let normalized = (z + 1.0) / 2.0;
        Vec4::new(
            0.5 + 0.5 * normalized,
            0.2 + 0.8 * normalized,
            0.8 - 0.8 * normalized,
            1.0,
        )
    });

    let sphere_id = scene.add_tattva(sphere, Vec3::new(4.0, 0.5, 0.0));

    scene.add_tattva(
        Label::new("Sphere Gradient", 0.16).with_color(GRAY_B),
        Vec3::new(4.0, -2.2, 0.0),
    );

    // Bottom: Torus with rainbow
    scene.add_tattva(
        Axes3D::new((-2.5, 2.5), (-2.5, 2.5), (-1.0, 1.0))
            .with_step(1.0)
            .with_axis_thickness(0.03),
        3.5 * DOWN,
    );

    let torus = ParametricSurface::new((0.0, 2.0 * PI), (0.0, 2.0 * PI), |u, v| {
        let r = 0.4;
        let major_radius = 1.2;
        let x = (major_radius + r * v.cos()) * u.cos();
        let y = (major_radius + r * v.cos()) * u.sin();
        let z = r * v.sin();
        Vec3::new(x, y, z)
    })
    .with_samples(28, 20)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_write_progress(0.0)
    .with_color_fn(|z| {
        let normalized = (z + 0.5) / 1.0;
        let hue = normalized * 6.0;
        match hue as i32 {
            0 => Vec4::new(1.0, normalized * 6.0, 0.0, 1.0),
            1 => Vec4::new(1.0 - (normalized - 1.0 / 6.0) * 6.0, 1.0, 0.0, 1.0),
            2 => Vec4::new(0.0, 1.0, (normalized - 2.0 / 6.0) * 6.0, 1.0),
            3 => Vec4::new(0.0, 1.0 - (normalized - 3.0 / 6.0) * 6.0, 1.0, 1.0),
            4 => Vec4::new((normalized - 4.0 / 6.0) * 6.0, 0.0, 1.0, 1.0),
            _ => Vec4::new(1.0, 0.0, 1.0 - (normalized - 5.0 / 6.0) * 6.0, 1.0),
        }
    });

    let torus_id = scene.add_tattva(torus, 3.5 * DOWN);

    scene.add_tattva(
        Label::new("Torus Rainbow", 0.16).with_color(GRAY_B),
        Vec3::new(0.0, -5.7, 0.0),
    );

    // Animations: staggered drawing
    let mut timeline = Timeline::new();

    // Wavy surface: draw from 0.0 to 2.0s
    timeline
        .animate(wavy_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .write_surface()
        .spawn();

    // Sphere: draw from 1.0 to 3.0s (overlapping)
    timeline
        .animate(sphere_id)
        .at(1.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .write_surface()
        .spawn();

    // Torus: draw from 2.0 to 4.0s (overlapping)
    timeline
        .animate(torus_id)
        .at(2.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .write_surface()
        .spawn();

    // Rotate all surfaces
    timeline
        .animate(wavy_id)
        .at(0.0)
        .for_duration(4.0)
        .ease(Ease::Linear)
        .rotate_to(glam::Quat::from_axis_angle(Vec3::Y, PI * 0.3))
        .spawn();

    timeline
        .animate(sphere_id)
        .at(0.0)
        .for_duration(4.0)
        .ease(Ease::Linear)
        .rotate_to(glam::Quat::from_axis_angle(Vec3::X, PI * 0.25))
        .spawn();

    timeline
        .animate(torus_id)
        .at(0.0)
        .for_duration(4.0)
        .ease(Ease::Linear)
        .rotate_to(glam::Quat::from_axis_angle(Vec3::Z, PI * 0.4))
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
