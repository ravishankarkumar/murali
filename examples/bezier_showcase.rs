use glam::{vec2, Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::path::Path;
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
        Label::new("Bézier Curves & Paths", 0.45).with_color(Vec4::new(0.9, 0.9, 0.9, 1.0)),
        Vec3::new(0.0, 3.0, 0.0),
    );

    // 1. Quadratic Bézier (Simple curve)
    let quad_path = Path::new()
        .with_thickness(0.08)
        .with_color(Vec4::new(0.35, 0.70, 0.92, 1.0)) // Blue
        .move_to(vec2(-2.0, 0.0))
        .quad_to(vec2(0.0, 2.0), vec2(2.0, 0.0));
    
    add_tattva(&mut scene, quad_path, Vec3::new(-3.0, 0.0, 0.0));

    // 2. Cubic Bézier (S-curve)
    let cubic_path = Path::new()
        .with_thickness(0.08)
        .with_color(Vec4::new(0.91, 0.58, 0.25, 1.0)) // Orange
        .move_to(vec2(-1.0, -1.0))
        .cubic_to(vec2(-1.0, 1.0), vec2(1.0, -1.0), vec2(1.0, 1.0));
    
    add_tattva(&mut scene, cubic_path, Vec3::new(0.0, 0.0, 0.0));

    // 3. Complex Closed Path (Heart-ish shape)
    let complex_path = Path::new()
        .with_thickness(0.06)
        .with_color(Vec4::new(0.90, 0.31, 0.25, 1.0)) // Red
        .move_to(vec2(0.0, -1.0))
        .cubic_to(vec2(-2.0, 1.0), vec2(-1.0, 2.0), vec2(0.0, 0.5))
        .cubic_to(vec2(1.0, 2.0), vec2(2.0, 1.0), vec2(0.0, -1.0))
        .close();
    
    add_tattva(&mut scene, complex_path, Vec3::new(3.0, -1.0, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
