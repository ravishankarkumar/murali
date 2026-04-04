use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::{
    circle::Circle, cube::Cube, line::Line, square::Square,
};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Square::new(1.8, Vec4::new(0.92, 0.26, 0.21, 1.0)),
        Vec3::new(-3.0, 1.2, 0.0),
    );

    scene.add_tattva(
        Circle::new(1.0, 48, Vec4::new(0.19, 0.64, 0.33, 1.0)),
        Vec3::new(0.0, 1.2, 0.0),
    );

    scene.add_tattva(
        Cube::new(1.6, Vec4::new(0.16, 0.50, 0.73, 1.0)),
        Vec3::new(3.0, 1.2, 0.0),
    );

    scene.add_tattva(
        Line::new(
            Vec3::new(-4.0, -1.6, 0.0),
            Vec3::new(4.0, -1.6, 0.0),
            0.08,
            Vec4::new(0.96, 0.80, 0.19, 1.0),
        ),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Line::new(
            Vec3::new(-3.5, -2.8, 0.0),
            Vec3::new(3.5, -0.4, 0.0),
            0.06,
            Vec4::new(0.61, 0.35, 0.71, 1.0),
        ),
        Vec3::ZERO,
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
