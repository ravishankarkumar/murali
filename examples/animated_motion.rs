use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let square_id = scene.add_tattva(
        Square::new(1.2, Vec4::new(0.92, 0.33, 0.29, 1.0)),
        Vec3::new(-4.0, 0.0, 0.0),
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.65, 48, Vec4::new(0.18, 0.65, 0.34, 1.0)),
        Vec3::new(4.0, -1.5, 0.0),
    );

    scene.add_tattva(
        Label::new("Timeline regression scene", 0.32).with_color(Vec4::new(0.92, 0.93, 0.94, 1.0)),
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
