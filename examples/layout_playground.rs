use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::{axes::Axes, number_plane::NumberPlane};
use murali::frontend::collection::layout::{Group, HStack, VStack};
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use murali::frontend::Tattva;
use murali::App;

fn add_tattva<T>(scene: &mut Scene, state: T, position: Vec3) -> usize
where
    T: murali::projection::Project + murali::frontend::layout::Bounded + Send + Sync + 'static,
{
    let tattva = Tattva::new(0, state);
    let id = scene.add(tattva);

    if let Some(t) = scene.get_tattva_any_mut(id) {
        let mut props = t.props().write();
        props.position = position;
    }

    id
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let plane_id = add_tattva(
        &mut scene,
        NumberPlane::new((-6.0, 6.0), (-3.5, 3.5)).with_step(1.0),
        Vec3::ZERO,
    );

    let axes_id = add_tattva(
        &mut scene,
        Axes::new((-6.0, 6.0), (-3.5, 3.5))
            .with_step(1.0)
            .with_thickness(0.03)
            .with_tick_size(0.18)
            .with_color(Vec4::new(0.95, 0.97, 0.99, 1.0)),
        Vec3::ZERO,
    );

    let title_id = add_tattva(
        &mut scene,
        Label::new("Layout Playground", 0.38).with_color(Vec4::new(0.96, 0.96, 0.97, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    let circle_id = add_tattva(
        &mut scene,
        Circle::new(0.45, 48, Vec4::new(0.19, 0.68, 0.35, 1.0)),
        Vec3::new(-3.0, 0.0, 0.0),
    );
    let square_id = add_tattva(
        &mut scene,
        Square::new(0.9, Vec4::new(0.90, 0.31, 0.25, 1.0)),
        Vec3::new(-1.0, 0.0, 0.0),
    );
    let node_label_id = add_tattva(
        &mut scene,
        Label::new("next_to + align_to", 0.24).with_color(Vec4::new(0.90, 0.91, 0.93, 1.0)),
        Vec3::ZERO,
    );
    scene.next_to(node_label_id, square_id, Direction::Up, 0.35);
    scene.align_to(node_label_id, square_id, murali::frontend::layout::Anchor::Left);

    let row_a = add_tattva(
        &mut scene,
        Label::new("Gradient", 0.28).with_color(Vec4::new(0.91, 0.58, 0.25, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let row_b = add_tattva(
        &mut scene,
        Label::new("Loss", 0.28).with_color(Vec4::new(0.35, 0.70, 0.92, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let row_c = add_tattva(
        &mut scene,
        Label::new("Weights", 0.28).with_color(Vec4::new(0.73, 0.45, 0.87, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );
    HStack::new(vec![row_a, row_b, row_c], 0.45).apply(&mut scene);
    Group::new(vec![row_a, row_b, row_c]).move_to(&mut scene, glam::vec2(2.5, 1.8));

    let col_a = add_tattva(
        &mut scene,
        Label::new("Input", 0.24).with_color(Vec4::new(0.90, 0.91, 0.93, 1.0)),
        Vec3::ZERO,
    );
    let col_b = add_tattva(
        &mut scene,
        Label::new("Hidden", 0.24).with_color(Vec4::new(0.90, 0.91, 0.93, 1.0)),
        Vec3::ZERO,
    );
    let col_c = add_tattva(
        &mut scene,
        Label::new("Output", 0.24).with_color(Vec4::new(0.90, 0.91, 0.93, 1.0)),
        Vec3::ZERO,
    );
    VStack::new(vec![col_a, col_b, col_c], 0.25).apply(&mut scene);
    Group::new(vec![col_a, col_b, col_c]).move_to(&mut scene, glam::vec2(4.4, -0.4));

    let _ = plane_id;
    let _ = axes_id;
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
