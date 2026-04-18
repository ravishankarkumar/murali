use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::{
    circle::Circle, polygon::Polygon, rectangle::Rectangle, square::Square,
};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // 1. Square to Circle
    let square_id = scene.add_tattva(
        Square::new(1.5, ORANGE_D),
        Vec3::new(-4.0, 1.5, 0.0),
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.8, 64, GREEN_C),
        Vec3::new(-4.0, 1.5, 0.0),
    );
    scene.hide_tattva(circle_id);

    // 2. Rectangle to Triangle (Polygon)
    let rect_id = scene.add_tattva(
        Rectangle::new(2.5, 1.2, BLUE_D),
        Vec3::new(2.0, 1.5, 0.0),
    );

    let triangle_id = scene.add_tattva(
        Polygon::regular(3, 1.0, YELLOW_B),
        Vec3::new(2.0, 1.5, 0.0),
    );
    scene.hide_tattva(triangle_id);

    // Create a timeline
    let mut timeline = Timeline::new();

    // Square -> Circle
    timeline
        .animate(circle_id)
        .at(0.5)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .morph_from(square_id)
        .spawn();

    // Rect -> Triangle
    timeline
        .animate(triangle_id)
        .at(1.5)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .morph_from(rect_id)
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
