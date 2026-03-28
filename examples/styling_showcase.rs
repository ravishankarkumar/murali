use murali::prelude::*;
use murali::engine::export::{ExportSettings, export_scene};
use glam::{vec2, Vec4};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // 1. Basic Colors & Opacity (Circle needs segments now)
    let red_rect = Rectangle::new(1.5, 1.0, Vec4::new(1.0, 0.2, 0.2, 0.8))
        .with_stroke(0.04, Vec4::new(1.0, 1.0, 1.0, 1.0));
    
    let green_circle = Circle::new(0.6, 32, Vec4::new(0.2, 0.8, 0.2, 0.6))
        .with_stroke(0.02, Vec4::new(0.8, 1.0, 0.8, 1.0));

    // 2. Dashing
    let dashed_path = Path::new()
        .move_to(vec2(-2.0, -1.5))
        .line_to(vec2(2.0, -1.5))
        .with_thickness(0.05)
        .with_color(Vec4::new(0.4, 0.7, 1.0, 1.0))
        .with_dash(0.2, 0.1);

    let dashed_circle = Circle::new(0.8, 48, Vec4::new(1.0, 1.0, 1.0, 0.1))
        .with_style(Style::new()
            .with_stroke(StrokeParams {
                thickness: 0.04,
                color: Vec4::new(0.9, 0.6, 1.0, 1.0),
                dash_length: 0.15,
                gap_length: 0.1,
                ..Default::default()
            })
        );

    // 3. Linear Gradient
    let gradient_rect = Rectangle::new(2.0, 1.0, Vec4::ONE)
        .with_style(Style::new().with_fill(ColorSource::LinearGradient {
            start: vec2(-1.0, 0.0),
            end: vec2(1.0, 0.0),
            stops: vec![
                (0.0, Vec4::new(0.1, 0.4, 0.9, 1.0)),
                (0.5, Vec4::new(0.8, 0.2, 0.8, 1.0)),
                (1.0, Vec4::new(0.9, 0.6, 0.1, 1.0)),
            ],
        }));

    // Add to scene and get IDs
    let r1 = scene.add(red_rect.into_tattva());
    let c1 = scene.add(green_circle.into_tattva());
    let gr = scene.add(gradient_rect.into_tattva());
    let dc = scene.add(dashed_circle.into_tattva());
    let dp = scene.add(dashed_path.into_tattva());

    // Layout
    let row1 = HStack::new(vec![r1, c1], 0.5);
    row1.apply(&mut scene);

    let row2 = HStack::new(vec![gr, dc], 0.5);
    row2.apply(&mut scene);

    let v_stack = VStack::new(vec![r1, gr, dp], 0.8);
    v_stack.apply(&mut scene);

    // Center camera
    scene.camera_mut().position = glam::Vec3::new(0.0, 0.0, 10.0);

    // Export to video
    let settings = ExportSettings {
        duration_seconds: 1.0,
        gif_path: Some("renders/styling_showcase.gif".into()),
        ..Default::default()
    };
    export_scene(scene, &settings)?;

    Ok(())
}
