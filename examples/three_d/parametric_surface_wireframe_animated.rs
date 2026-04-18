/// Animated Parametric Surface with Progressive Wireframe Drawing
/// Grid lines are drawn in real-time: horizontal lines first, then vertical lines
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
        Label::new("Animated Wireframe Surface", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // 3D Axes
    scene.add_tattva(
        Axes3D::new((-2.0, 2.0), (-2.0, 2.0), (-1.0, 1.0))
            .with_step(1.0)
            .with_axis_thickness(0.04),
        ORIGIN,
    );

    // Wavy surface with animated wireframe (starts invisible)
    let wavy_animated = ParametricSurface::new((-PI, PI), (-PI, PI), |u, v| {
        let x = u;
        let y = v;
        let z = (u * 0.8).sin() * (v * 0.8).cos();
        Vec3::new(x, y, z)
    })
    .with_samples(28, 28)
    .with_render_mode(SurfaceRenderMode::Wireframe)
    .with_write_progress(0.0) // Start invisible
    .with_color_fn(|z| {
        // Heat map: blue (cold) to red (hot)
        let normalized = (z + 1.0) / 2.0;
        if normalized < 0.5 {
            let t = normalized * 2.0;
            Vec4::new(0.0, t, 1.0, 1.0) // Blue to cyan
        } else {
            let t = (normalized - 0.5) * 2.0;
            Vec4::new(t, 1.0 - t, 0.0, 1.0) // Cyan to red
        }
    });

    let surface_id = scene.add_tattva(wavy_animated, ORIGIN);

    // Subtitle
    scene.add_tattva(
        Label::new(
            "Grid lines drawn progressively: horizontal → vertical",
            0.18,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -2.2, 0.0),
    );

    // Animation: draw the wireframe progressively
    let mut timeline = Timeline::new();

    // Phase 1 (0.0-2.0s): Draw horizontal lines (0.0 to 0.5 progress)
    // Phase 2 (2.0-4.0s): Draw vertical lines (0.5 to 1.0 progress)
    timeline
        .animate(surface_id)
        .at(0.0)
        .for_duration(4.0)
        .ease(Ease::InOutQuad)
        .write_surface()
        .spawn();

    // Optional: Rotate the surface while drawing
    timeline
        .animate(surface_id)
        .at(0.0)
        .for_duration(4.0)
        .ease(Ease::Linear)
        .rotate_to(glam::Quat::from_axis_angle(Vec3::Z, PI * 0.5))
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
