use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::circle::Circle;
use glam::{Vec3, Vec4};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Circle::new(1.0, 64, Vec4::new(0.2, 0.6, 1.0, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}