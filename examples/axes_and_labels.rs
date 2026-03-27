use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::axes::Axes;
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

    let mut axes = Axes::new((-5.0, 5.0), (-3.0, 3.0));
    axes.x_step = 1.0;
    axes.y_step = 1.0;
    axes.thickness = 0.03;
    axes.tick_size = 0.18;
    axes.color = Vec4::new(0.75, 0.79, 0.85, 1.0);
    add_tattva(&mut scene, axes, Vec3::ZERO);

    add_tattva(
        &mut scene,
        Label::new("Murali Axes", 0.45).with_color(Vec4::new(0.98, 0.98, 0.98, 1.0)),
        Vec3::new(0.0, 3.7, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("x", 0.32).with_color(Vec4::new(0.93, 0.42, 0.37, 1.0)),
        Vec3::new(5.35, -0.15, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("y", 0.32).with_color(Vec4::new(0.37, 0.68, 0.91, 1.0)),
        Vec3::new(0.18, 3.35, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("Regression scene: axes + text", 0.24)
            .with_color(Vec4::new(0.80, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.65, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
