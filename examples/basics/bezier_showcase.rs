use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::path::Path;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    scene.add_tattva(
        Label::new("Bézier Curves & Paths", 0.45).with_color(GRAY_B),
        3.0 * UP,
    );

    // 1. Quadratic Bézier (Simple curve)
    let quad_path = Path::new()
        .with_thickness(0.08)
        .with_color(BLUE_B) // Blue
        .move_to(vec2(-2.0, 0.0))
        .quad_to(vec2(0.0, 2.0), vec2(2.0, 0.0));

    scene.add_tattva(quad_path, 3.0 * LEFT);

    // 2. Cubic Bézier (S-curve)
    let cubic_path = Path::new()
        .with_thickness(0.08)
        .with_color(GOLD_C) // Orange
        .move_to(vec2(-1.0, -1.0))
        .cubic_to(vec2(-1.0, 1.0), vec2(1.0, -1.0), vec2(1.0, 1.0));

    scene.add_tattva(cubic_path, ORIGIN);

    // 3. Complex Closed Path (Heart-ish shape)
    let complex_path = Path::new()
        .with_thickness(0.06)
        .with_color(RED_B) // Red
        .move_to(vec2(0.0, -1.0))
        .cubic_to(vec2(-2.0, 1.0), vec2(-1.0, 2.0), vec2(0.0, 0.5))
        .cubic_to(vec2(1.0, 2.0), vec2(2.0, 1.0), vec2(0.0, -1.0))
        .close();

    scene.add_tattva(complex_path, Vec3::new(3.0, -1.0, 0.0));

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
