use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, rectangle::Rectangle, square::Square, polygon::Polygon};
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

    // 1. Square to Circle
    let square_id = add_tattva(
        &mut scene,
        Square::new(1.5, Vec4::new(0.96, 0.42, 0.28, 1.0)),
        Vec3::new(-4.0, 1.5, 0.0),
    );

    let circle_id = add_tattva(
        &mut scene,
        Circle::new(0.8, 64, Vec4::new(0.26, 0.70, 0.44, 1.0)),
        Vec3::new(-4.0, 1.5, 0.0),
    );
    // Hide the target shape initially
    if let Some(t) = scene.get_tattva_any_mut(circle_id) {
        let mut props = t.props().write();
        props.visible = false;
        props.opacity = 0.0;
    }

    // 2. Rectangle to Triangle (Polygon)
    let rect_id = add_tattva(
        &mut scene,
        Rectangle::new(2.5, 1.2, Vec4::new(0.22, 0.50, 0.96, 1.0)),
        Vec3::new(2.0, 1.5, 0.0),
    );

    let triangle_id = add_tattva(
        &mut scene,
        Polygon::regular(3, 1.0, Vec4::new(0.98, 0.66, 0.22, 1.0)),
        Vec3::new(2.0, 1.5, 0.0),
    );
    if let Some(t) = scene.get_tattva_any_mut(triangle_id) {
        let mut props = t.props().write();
        props.visible = false;
        props.opacity = 0.0;
    }

    // Create a timeline
    let mut timeline = Timeline::new();

    // Square -> Circle
    timeline
        .animate(circle_id)
        .at(0.5)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .morph_from(square_id)
        .spawn();

    // Rect -> Triangle
    timeline
        .animate(triangle_id)
        .at(1.5)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .morph_from(rect_id)
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    println!("Running morph showcase...");
    println!("0.5s: Square → Circle");
    println!("1.5s: Rectangle → Triangle");

    App::new()?.with_scene(scene).run_app()
}
