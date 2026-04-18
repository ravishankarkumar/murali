use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::text::label::Label;
use murali::frontend::style::Style;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let flute_body = GOLD_D;
    let flute_edge = GOLD_E;
    let hole = BLACK;
    let accent = GOLD_B;
    let text = WHITE;
    let subtitle = GRAY_A;

    let body_style =
        Style::new()
            .with_fill(flute_body)
            .with_stroke(murali::frontend::style::StrokeParams {
                thickness: 0.03,
                color: flute_edge,
                ..Default::default()
            });

    scene.add_tattva(
        Rectangle::new(5.2, 0.44, flute_body).with_style(body_style),
        Vec3::new(-2.6, 0.15, 0.0),
    );

    scene.add_tattva(
        Circle::new(0.22, 36, flute_body).with_stroke(0.03, flute_edge),
        Vec3::new(-5.2, 0.15, 0.0),
    );

    scene.add_tattva(
        Circle::new(0.22, 36, flute_body).with_stroke(0.03, flute_edge),
        0.15 * UP,
    );

    scene.add_tattva(
        Circle::new(0.10, 28, accent).with_stroke(0.02, flute_edge),
        Vec3::new(-4.45, 0.15, 0.0),
    );

    for x in [-3.45_f32, -2.60, -1.75, -0.90, -0.05] {
        scene.add_tattva(
            Circle::new(0.11, 28, hole).with_stroke(0.01, Vec4::new(BLACK.x, BLACK.y, BLACK.z, 0.45)),
            Vec3::new(x, 0.15, 0.0),
        );
    }

    scene.add_tattva(
        Rectangle::new(0.08, 0.58, accent),
        Vec3::new(-4.95, 0.15, 0.0),
    );

    scene.add_tattva(
        Rectangle::new(0.08, 0.58, accent),
        Vec3::new(-0.28, 0.15, 0.0),
    );

    scene.add_tattva(
        Label::new("MURALI", 0.82).with_color(text),
        Vec3::new(4.15, 0.50, 0.0),
    );

    scene.add_tattva(
        Label::new("BANSURI OF MATHEMATICAL MOTION", 0.20).with_color(subtitle),
        Vec3::new(4.25, -0.62, 0.0),
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    App::new()?.with_scene(scene).run_app()
}
