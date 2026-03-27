use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;
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

    let square_id = add_tattva(
        &mut scene,
        Square::new(1.2, Vec4::new(0.92, 0.33, 0.29, 1.0)),
        Vec3::new(-4.0, 0.0, 0.0),
    );

    let circle_id = add_tattva(
        &mut scene,
        Circle::new(0.65, 48, Vec4::new(0.18, 0.65, 0.34, 1.0)),
        Vec3::new(4.0, -1.5, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("Timeline regression scene", 0.32)
            .with_color(Vec4::new(0.92, 0.93, 0.94, 1.0)),
        Vec3::new(0.0, 3.0, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(square_id)
        .at(0.0)
        .for_duration(2.2)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(2.6, 0.8, 0.0))
        .spawn();

    timeline
        .animate(circle_id)
        .at(0.4)
        .for_duration(2.6)
        .ease(Ease::OutQuad)
        .move_to(Vec3::new(-2.5, 1.3, 0.0))
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
