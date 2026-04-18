use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::axes::Axes;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let mut axes = Axes::new((-5.0, 5.0), (-3.0, 3.0));
    axes.x_step = 1.0;
    axes.y_step = 1.0;
    axes.thickness = 0.03;
    axes.tick_size = 0.18;
    axes.color = GRAY_A;
    scene.add_tattva(axes, Vec3::ZERO);

    scene.add_tattva(
        Label::new("Murali Axes", 0.45).with_color(WHITE),
        3.7 * UP,
    );

    scene.add_tattva(
        Label::new("x", 0.32).with_color(ORANGE_B),
        5.35 * RIGHT + 0.15 * DOWN,
    );

    scene.add_tattva(
        Label::new("y", 0.32).with_color(BLUE_B),
        0.18 * RIGHT + 3.35 * UP,
    );

    scene.add_tattva(
        Label::new("Regression scene: axes + text", 0.24)
            .with_color(GRAY_A),
        3.65 * DOWN,
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
