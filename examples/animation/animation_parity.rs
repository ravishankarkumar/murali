use glam::{Quat, Vec3};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::composite::number_plane::NumberPlane;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Anchor;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        NumberPlane::new((-6.0, 6.0), (-3.5, 3.5)).with_step(1.0),
        Vec3::ZERO,
    );

    let square_id = scene.add_tattva(
        Square::new(1.0, RED_B),
        Vec3::new(-4.5, -1.0, 0.0),
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.55, 48, GREEN_D),
        Vec3::new(2.0, 0.5, 0.0),
    );

    let label_id = scene.add_tattva(
        Label::new("Follow + fade/appear", 0.28).with_color(WHITE),
        ORIGIN,
    );

    // let subtitle_id = scene.add_tattva(
    //     Label::new("Milestone 3 regression scene", 0.24)
    //         .with_color(GRAY_A),
    //     Vec3::new(0.0, 3.2, 0.0),
    // );

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
        .appear()
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
        .follow_anchor(
            circle_id,
            Anchor::Up,
            Anchor::Down,
            Vec3::new(0.0, 0.35, 0.0),
        )
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

    // timeline
    //     .animate(subtitle_id)
    //     .at(2.6)
    //     .for_duration(1.2)
    //     .ease(Ease::InOutQuad)
    //     .fade_to(0.25)
    //     .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
