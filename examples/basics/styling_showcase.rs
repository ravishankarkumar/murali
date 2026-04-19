use glam::{Vec4, vec2};
use murali::engine::export::{ExportSettings, export_scene};
use murali::positions::*;
use murali::colors::*;
use murali::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // 1. Basic Colors & Opacity (Circle needs segments now)
    let red_rect = Rectangle::new(1.5, 1.0, Vec4::new(RED_B.x, RED_B.y, RED_B.z, 0.8))
        .with_stroke(0.04, WHITE);

    let green_circle = Circle::new(0.6, 32, Vec4::new(GREEN_B.x, GREEN_B.y, GREEN_B.z, 0.6))
        .with_stroke(0.02, GREEN_B);

    // 2. Dashing
    let dashed_path = Path::new()
        .move_to(vec2(-2.0, -1.5))
        .line_to(vec2(2.0, -1.5))
        .with_thickness(0.05)
        .with_color(BLUE_B)
        .with_dash(0.2, 0.1);

    let dashed_circle = Circle::new(0.8, 48, Vec4::new(WHITE.x, WHITE.y, WHITE.z, 0.1)).with_style(
        Style::new().with_stroke(StrokeParams {
            thickness: 0.04,
            color: PURPLE_B,
            dash_length: 0.15,
            gap_length: 0.1,
            ..Default::default()
        }),
    );

    // 3. Linear Gradient
    let gradient_rect = Rectangle::new(2.0, 1.0, Vec4::ONE).with_style(Style::new().with_fill(
        ColorSource::LinearGradient {
            start: vec2(-1.0, 0.0),
            end: vec2(1.0, 0.0),
            stops: vec![
                (0.0, BLUE_D),
                (0.5, PURPLE_B),
                (1.0, GOLD_C),
            ],
        },
    ));

    // Add to scene and get IDs
    let r1 = scene.add_tattva(red_rect, glam::Vec3::ZERO);
    let c1 = scene.add_tattva(green_circle, glam::Vec3::ZERO);
    let gr = scene.add_tattva(gradient_rect, glam::Vec3::ZERO);
    let dc = scene.add_tattva(dashed_circle, glam::Vec3::ZERO);
    let dp = scene.add_tattva(dashed_path, glam::Vec3::ZERO);

    // Layout
    let row1 = HStack::new(vec![r1, c1], 0.5);
    row1.apply(&mut scene);

    let row2 = HStack::new(vec![gr, dc], 0.5);
    row2.apply(&mut scene);

    let v_stack = VStack::new(vec![r1, gr, dp], 0.8);
    v_stack.apply(&mut scene);

    // Center camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // Export to video
    let settings = ExportSettings {
        duration_seconds: 1.0,
        gif_path: Some("render_output/styling_showcase.gif".into()),
        ..Default::default()
    };
    export_scene(scene, &settings)?;

    Ok(())
}
