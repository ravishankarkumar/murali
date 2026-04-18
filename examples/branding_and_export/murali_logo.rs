use glam::{Vec2, Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::arrow::Arrow;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::line::Line;
use murali::frontend::collection::primitives::path::Path;
use murali::frontend::collection::text::label::Label;

fn add_line(scene: &mut Scene, start: Vec2, end: Vec2, thickness: f32, color: Vec4) {
    scene.add_tattva(
        Line::new(
            Vec3::new(start.x, start.y, 0.0),
            Vec3::new(end.x, end.y, 0.0),
            thickness,
            color,
        ),
        Vec3::ZERO,
    );
}

fn add_point(scene: &mut Scene, position: Vec2, radius: f32, color: Vec4) {
    scene.add_tattva(
        Circle::new(radius, 28, color),
        Vec3::new(position.x, position.y, 0.0),
    );
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let cyan = TEAL_B;
    let blue = BLUE_D;
    let grid = Vec4::new(BLUE_D.x, BLUE_D.y, BLUE_D.z, 0.55);
    let axis = Vec4::new(BLUE_B.x, BLUE_B.y, BLUE_B.z, 0.95);
    let white = WHITE;
    let soft = GRAY_A;

    let symbol_offset = Vec2::new(-3.5, 0.1);
    let half = 2.1;
    let frame_thickness = 0.035;

    let left = symbol_offset.x - half;
    let right = symbol_offset.x + half;
    let top = symbol_offset.y + half;
    let bottom = symbol_offset.y - half;

    add_line(
        &mut scene,
        vec2(left, bottom),
        vec2(left, top),
        frame_thickness,
        grid,
    );
    add_line(
        &mut scene,
        vec2(left, top),
        vec2(right, top),
        frame_thickness,
        grid,
    );
    add_line(
        &mut scene,
        vec2(right, top),
        vec2(right, bottom),
        frame_thickness,
        grid,
    );
    add_line(
        &mut scene,
        vec2(right, bottom),
        vec2(left, bottom),
        frame_thickness,
        grid,
    );

    for x in [-1.0_f32, 1.0] {
        add_line(
            &mut scene,
            vec2(symbol_offset.x + x, bottom),
            vec2(symbol_offset.x + x, top),
            0.02,
            Vec4::new(grid.x, grid.y, grid.z, 0.38),
        );
    }

    for y in [-1.0_f32, 1.0] {
        add_line(
            &mut scene,
            vec2(left, symbol_offset.y + y),
            vec2(right, symbol_offset.y + y),
            0.02,
            Vec4::new(grid.x, grid.y, grid.z, 0.38),
        );
    }

    scene.add_tattva(
        Arrow::with_default_tip(
            vec2(left - 0.45, symbol_offset.y),
            vec2(right + 0.55, symbol_offset.y),
            0.035,
            axis,
        ),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Arrow::with_default_tip(
            vec2(symbol_offset.x, bottom - 0.45),
            vec2(symbol_offset.x, top + 0.55),
            0.035,
            axis,
        ),
        Vec3::ZERO,
    );

    let m_path = Path::new()
        .with_thickness(0.10)
        .with_color(cyan)
        .move_to(vec2(-1.55, -1.35))
        .cubic_to(vec2(-1.42, 0.95), vec2(-1.05, 1.45), vec2(-0.35, 0.15))
        .line_to(vec2(0.0, -0.08))
        .line_to(vec2(1.05, 1.35))
        .cubic_to(vec2(1.40, 1.78), vec2(1.58, 0.88), vec2(1.55, -0.62));
    scene.add_tattva(m_path, Vec3::new(symbol_offset.x, symbol_offset.y, 0.0));

    let lower_arc = Path::new()
        .with_thickness(0.08)
        .with_color(blue)
        .move_to(vec2(-1.18, -1.02))
        .cubic_to(vec2(-0.52, -1.75), vec2(0.72, -1.88), vec2(1.55, -0.72));
    scene.add_tattva(lower_arc, Vec3::new(symbol_offset.x, symbol_offset.y, 0.0));

    let inner_wave = Path::new()
        .with_thickness(0.06)
        .with_color(Vec4::new(BLUE_B.x, BLUE_B.y, BLUE_B.z, 0.92))
        .move_to(vec2(-1.02, 0.42))
        .cubic_to(vec2(-0.72, -0.18), vec2(-0.18, -0.92), vec2(0.52, -0.62))
        .cubic_to(vec2(0.96, -0.42), vec2(1.18, 0.18), vec2(1.32, 0.58));
    scene.add_tattva(inner_wave, Vec3::new(symbol_offset.x, symbol_offset.y, 0.0));

    for point in [
        vec2(-1.55, -1.35),
        vec2(-1.30, 1.45),
        vec2(0.0, -0.08),
        vec2(1.05, 1.35),
        vec2(1.55, -0.72),
        vec2(-1.18, -1.02),
        vec2(1.32, 0.58),
    ] {
        let t = ((point.x + 1.6) / 3.2).clamp(0.0, 1.0);
        add_point(&mut scene, point + symbol_offset, 0.09, blue.lerp(cyan, t));
    }

    scene.add_tattva(
        Label::new("MURALI", 0.88).with_color(white),
        Vec3::new(3.7, 0.42, 0.0),
    );

    scene.add_tattva(
        Label::new("MATHEMATICAL ANIMATION ENGINE", 0.23).with_color(soft),
        Vec3::new(4.2, -0.95, 0.0),
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    App::new()?.with_scene(scene).run_app()
}
