use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use glam::{Vec3, Vec4};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    let circle_id = scene.add_tattva(
        Circle::new(1.5, 48, Vec4::new(0.19, 0.64, 0.33, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    timeline.animate(circle_id).at(0.0).for_duration(1.5).ease(Ease::OutCubic).write().spawn();
    timeline.animate(circle_id).at(3.0).for_duration(1.0).ease(Ease::InCubic).unwrite().spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
