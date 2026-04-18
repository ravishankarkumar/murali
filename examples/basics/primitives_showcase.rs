use glam::Vec3;
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::Palette;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::{
    circle::Circle, cube::Cube, line::Line, square::Square,
};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let palette = Palette::load("layers");

    scene.add_tattva(
        Square::new(1.8, palette.require("layer-0")),
        Vec3::new(-3.0, 1.2, 0.0),
    );

    scene.add_tattva(
        Circle::new(1.0, 48, palette.require("layer-1")),
        Vec3::new(0.0, 1.2, 0.0),
    );

    scene.add_tattva(
        Cube::new(1.6, palette.require("layer-2")),
        Vec3::new(3.0, 1.2, 0.0),
    );

    scene.add_tattva(
        Line::new(
            Vec3::new(-4.0, -1.6, 0.0),
            Vec3::new(4.0, -1.6, 0.0),
            0.08,
            palette.cycle_or_panic(3),
        ),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Line::new(
            Vec3::new(-3.5, -2.8, 0.0),
            Vec3::new(3.5, -0.4, 0.0),
            0.06,
            palette.cycle_or_panic(4),
        ),
        Vec3::ZERO,
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}