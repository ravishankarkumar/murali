/// Arrow Primitive Showcase
/// Demonstrates the Arrow primitive with various configurations
use glam::{Vec2, Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::arrow::Arrow;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Arrow Primitive Showcase", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Simple arrows in different directions
    let arrow_up = Arrow::with_default_tip(
        Vec2::new(-5.0, -1.0),
        Vec2::new(-5.0, 1.0),
        0.05,
        RED_C,
    );
    scene.add_tattva(arrow_up, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Up", 0.14).with_color(GRAY_B),
        Vec3::new(-5.0, -1.8, 0.0),
    );

    let arrow_right = Arrow::with_default_tip(
        Vec2::new(-3.5, 0.0),
        Vec2::new(-1.5, 0.0),
        0.05,
        GREEN_B,
    );
    scene.add_tattva(arrow_right, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Right", 0.14).with_color(GRAY_B),
        Vec3::new(-2.5, -1.8, 0.0),
    );

    let arrow_diagonal = Arrow::with_default_tip(
        Vec2::new(0.0, -1.0),
        Vec2::new(1.5, 1.0),
        0.05,
        BLUE_B,
    );
    scene.add_tattva(arrow_diagonal, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Diagonal", 0.14).with_color(GRAY_B),
        Vec3::new(0.75, -1.8, 0.0),
    );

    // Different sizes
    let small_arrow = Arrow::with_default_tip(
        Vec2::new(3.0, -0.5),
        Vec2::new(3.0, 0.5),
        0.03,
        GOLD_C,
    );
    scene.add_tattva(small_arrow, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Small", 0.14).with_color(GRAY_B),
        Vec3::new(3.0, -1.8, 0.0),
    );

    let large_arrow = Arrow::with_default_tip(
        Vec2::new(4.5, -1.5),
        Vec2::new(4.5, 1.5),
        0.08,
        PURPLE_B,
    );
    scene.add_tattva(large_arrow, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Large", 0.14).with_color(GRAY_B),
        Vec3::new(4.5, -1.8, 0.0),
    );

    // Custom tip proportions
    let wide_tip = Arrow::new(
        Vec2::new(-5.0, 2.0),
        Vec2::new(-3.0, 2.0),
        0.04,
        0.3, // tip length
        0.4, // tip width (wide)
        RED_B,
    );
    scene.add_tattva(wide_tip, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Wide Tip", 0.14).with_color(GRAY_B),
        4.0 * LEFT + 1.2 * UP,
    );

    let narrow_tip = Arrow::new(
        Vec2::new(-2.0, 2.0),
        Vec2::new(0.0, 2.0),
        0.04,
        0.4, // tip length (long)
        0.1, // tip width (narrow)
        GREEN_B,
    );
    scene.add_tattva(narrow_tip, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Narrow Tip", 0.14).with_color(GRAY_B),
        Vec3::new(-1.0, 1.2, 0.0),
    );

    let long_tip = Arrow::new(
        Vec2::new(1.0, 2.0),
        Vec2::new(3.0, 2.0),
        0.04,
        0.6, // tip length (very long)
        0.2, // tip width
        BLUE_B,
    );
    scene.add_tattva(long_tip, Vec3::ZERO);
    scene.add_tattva(
        Label::new("Long Tip", 0.14).with_color(GRAY_B),
        2.0 * RIGHT + 1.2 * UP,
    );

    // Vector field pattern
    scene.add_tattva(
        Label::new("Vector Field Pattern", 0.14).with_color(GRAY_B),
        Vec3::new(0.0, -2.8, 0.0),
    );

    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
        let start = Vec2::ZERO;
        let end = Vec2::new(angle.cos() * 0.8, angle.sin() * 0.8);

        let arrow = Arrow::with_default_tip(start, end, 0.03, Vec4::new(BLUE_A.x, BLUE_A.y, BLUE_A.z, 0.9));
        scene.add_tattva(arrow, Vec3::new(4.5, -3.5, 0.0));
    }

    App::new()?.with_scene(scene).run_app()
}
