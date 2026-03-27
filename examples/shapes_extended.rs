use glam::{vec2, Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::primitives::polygon::Polygon;
use murali::frontend::collection::primitives::ellipse::Ellipse;
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

    // Title
    add_tattva(
        &mut scene,
        Label::new("Extended Shapes: Rectangles & Polygons", 0.45).with_color(Vec4::new(0.9, 0.9, 0.9, 1.0)),
        Vec3::new(0.0, 3.0, 0.0),
    );

    // 1. Rectangle
    let rect = Rectangle::new(3.0, 1.2, Vec4::new(0.91, 0.58, 0.25, 1.0)); // Orange
    add_tattva(&mut scene, rect, Vec3::new(-3.0, 0.0, 0.0));

    // 2. Regular Hexagon (via Polygon::regular)
    let hexagon = Polygon::regular(6, 1.0, Vec4::new(0.35, 0.70, 0.92, 1.0)); // Blue
    add_tattva(&mut scene, hexagon, Vec3::new(0.0, 0.0, 0.0));

    // 4. Ellipse
    let ellipse = Ellipse::new(1.2, 0.6, Vec4::new(0.8, 0.4, 0.8, 1.0)); // Purple
    add_tattva(&mut scene, ellipse, Vec3::new(0.0, -1.8, 0.0));

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
    add_tattva(&mut scene, custom_poly, Vec3::new(3.0, 0.0, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
