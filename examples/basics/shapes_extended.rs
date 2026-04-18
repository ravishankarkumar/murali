use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::ellipse::Ellipse;
use murali::frontend::collection::primitives::polygon::Polygon;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    scene.add_tattva(
        Label::new("Extended Shapes: Rectangles & Polygons", 0.45)
            .with_color(GRAY_B),
        3.0 * UP,
    );

    // 1. Rectangle
    let rect = Rectangle::new(3.0, 1.2, GOLD_C); // Orange
    scene.add_tattva(rect, 3.0 * LEFT);

    // 2. Regular Hexagon (via Polygon::regular)
    let hexagon = Polygon::regular(6, 1.0, BLUE_B); // Blue
    scene.add_tattva(hexagon, ORIGIN);

    // 4. Ellipse
    let ellipse = Ellipse::new(1.2, 0.6, PURPLE_B); // Purple
    scene.add_tattva(ellipse, Vec3::new(0.0, -1.8, 0.0));

    // 3. Custom Convex Polygon
    let custom_poly = Polygon::new(
        vec![
            vec2(0.0, 1.2),
            vec2(1.2, 0.0),
            vec2(0.8, -1.2),
            vec2(-0.8, -1.2),
            vec2(-1.2, 0.0),
        ],
        GREEN_D, // Green
    );
    scene.add_tattva(custom_poly, 3.0 * RIGHT);

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
