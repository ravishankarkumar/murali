use glam::{Quat, Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::composite::number_plane::NumberPlane;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Anchor;
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

    add_tattva(
        &mut scene,
        NumberPlane::new((-6.0, 6.0), (-3.5, 3.5)).with_step(1.0),
        Vec3::ZERO,
    );

    let square_id = add_tattva(
        &mut scene,
        Square::new(1.0, Vec4::new(0.92, 0.32, 0.27, 1.0)),
        Vec3::new(-4.5, -1.0, 0.0),
    );

    let circle_id = add_tattva(
        &mut scene,
        Circle::new(0.55, 48, Vec4::new(0.20, 0.67, 0.37, 1.0)),
        Vec3::new(2.0, 0.5, 0.0),
    );

    let label_id = add_tattva(
        &mut scene,
        Label::new("Follow + fade/create", 0.28).with_color(Vec4::new(0.95, 0.96, 0.97, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let subtitle_id = add_tattva(
        &mut scene,
        Label::new("Milestone 3 regression scene", 0.24)
            .with_color(Vec4::new(0.82, 0.84, 0.87, 1.0)),
        Vec3::new(0.0, 3.2, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(square_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(-1.5, 1.2, 0.0))
        .spawn();

    timeline
        .animate(square_id)
        .at(0.3)
        .for_duration(2.4)
        .ease(Ease::InOutQuad)
        .rotate_to(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
        .spawn();

    timeline
        .animate(square_id)
        .at(0.4)
        .for_duration(2.2)
        .ease(Ease::OutQuad)
        .scale_to(Vec3::splat(1.8))
        .spawn();

    timeline
        .animate(circle_id)
        .at(0.0)
        .for_duration(1.4)
        .ease(Ease::Linear)
        .create()
        .spawn();

    timeline
        .animate(circle_id)
        .at(2.4)
        .for_duration(1.6)
        .ease(Ease::InOutQuad)
        .fade_to(0.15)
        .spawn();

    timeline
        .animate(label_id)
        .at(0.0)
        .for_duration(8.0)
        .ease(Ease::Linear)
        .follow_anchor(circle_id, Anchor::Up, Anchor::Down, Vec3::new(0.0, 0.35, 0.0))
        .spawn();

    timeline
        .animate_camera()
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .frame_to(Vec3::new(0.8, 0.4, 9.5), Vec3::new(0.3, 0.2, 0.0))
        .spawn();

    timeline
        .animate_camera()
        .at(2.0)
        .for_duration(1.5)
        .ease(Ease::OutQuad)
        .zoom_to(1.35)
        .spawn();

    timeline
        .animate(subtitle_id)
        .at(2.6)
        .for_duration(1.2)
        .ease(Ease::InOutQuad)
        .fade_to(0.25)
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
