use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::graph::parametric_surface::ParametricSurface;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Textured Globe Surface", 0.40).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new("Image texture wrapped onto a parametric sphere", 0.18)
            .with_color(GRAY_B),
        Vec3::new(0.0, 2.95, 0.0),
    );

    // Content
    let textured_globe = ParametricSurface::new((0.0, PI), (0.0, 2.0 * PI), |u, v| {
        let sin_u = u.sin();
        Vec3::new(sin_u * v.cos(), u.cos(), sin_u * v.sin())
    })
    .with_samples(48, 72)
    .with_color(Vec4::ONE);

    scene.add_textured_surface_with_path(
        textured_globe,
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/resource/assets/earthmap1k.jpg"
        ),
        Vec3::new(0.0, 0.05, 0.0),
    )?;

    // Supporting label
    scene.add_tattva(
        Label::new(
            "Earth map wrapped onto a spherical parametric surface",
            0.18,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -1.95, 0.0),
    );

    // Camera
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 5.6);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}
