use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::{axes::Axes, number_plane::NumberPlane};
use murali::frontend::collection::layout::{Group, HStack, VStack};
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let plane_id = scene.add_tattva(
        NumberPlane::new((-6.0, 6.0), (-3.5, 3.5)).with_step(1.0),
        Vec3::ZERO,
    );

    let axes_id = scene.add_tattva(
        Axes::new((-6.0, 6.0), (-3.5, 3.5))
            .with_step(1.0)
            .with_thickness(0.03)
            .with_tick_size(0.18)
            .with_color(WHITE),
        Vec3::ZERO,
    );

    let title_id = scene.add_tattva(
        Label::new("Layout Playground", 0.38).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    let _circle_id = scene.add_tattva(
        Circle::new(0.45, 48, GREEN_D),
        3.0 * LEFT,
    );
    let square_id = scene.add_tattva(
        Square::new(0.9, RED_B),
        LEFT,
    );
    let node_label_id = scene.add_tattva(
        Label::new("next_to + align_to", 0.24).with_color(GRAY_A),
        Vec3::ZERO,
    );
    scene.next_to(node_label_id, square_id, Direction::Up, 0.35);
    scene.align_to(
        node_label_id,
        square_id,
        murali::frontend::layout::Anchor::Left,
    );

    let row_a = scene.add_tattva(
        Label::new("Gradient", 0.28).with_color(GOLD_C),
        ORIGIN,
    );
    let row_b = scene.add_tattva(
        Label::new("Loss", 0.28).with_color(BLUE_B),
        ORIGIN,
    );
    let row_c = scene.add_tattva(
        Label::new("Weights", 0.28).with_color(PURPLE_B),
        ORIGIN,
    );
    HStack::new(vec![row_a, row_b, row_c], 0.45).apply(&mut scene);
    Group::new(vec![row_a, row_b, row_c]).move_to(&mut scene, glam::vec2(2.5, 1.8));

    let col_a = scene.add_tattva(
        Label::new("Input", 0.24).with_color(GRAY_A),
        Vec3::ZERO,
    );
    let col_b = scene.add_tattva(
        Label::new("Hidden", 0.24).with_color(GRAY_A),
        Vec3::ZERO,
    );
    let col_c = scene.add_tattva(
        Label::new("Output", 0.24).with_color(GRAY_A),
        Vec3::ZERO,
    );
    VStack::new(vec![col_a, col_b, col_c], 0.25).apply(&mut scene);
    Group::new(vec![col_a, col_b, col_c]).move_to(&mut scene, glam::vec2(4.4, -0.4));

    let _ = plane_id;
    let _ = axes_id;
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
