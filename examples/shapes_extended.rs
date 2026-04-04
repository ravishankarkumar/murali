use glam::{Vec3, Vec4, vec2};
use murali::App;
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
            .with_color(Vec4::new(0.9, 0.9, 0.9, 1.0)),
        Vec3::new(0.0, 3.0, 0.0),
    );

    // 1. Rectangle
    let rect = Rectangle::new(3.0, 1.2, Vec4::new(0.91, 0.58, 0.25, 1.0)); // Orange
    scene.add_tattva(rect, Vec3::new(-3.0, 0.0, 0.0));

    // 2. Regular Hexagon (via Polygon::regular)
    let hexagon = Polygon::regular(6, 1.0, Vec4::new(0.35, 0.70, 0.92, 1.0)); // Blue
    scene.add_tattva(hexagon, Vec3::new(0.0, 0.0, 0.0));

    // 4. Ellipse
    let ellipse = Ellipse::new(1.2, 0.6, Vec4::new(0.8, 0.4, 0.8, 1.0)); // Purple
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
        Vec4::new(0.19, 0.68, 0.35, 1.0), // Green
    );
    scene.add_tattva(custom_poly, Vec3::new(3.0, 0.0, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
