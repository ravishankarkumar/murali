use glam::{Vec2, Vec3, Vec4, vec2, vec3};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::{DrawableProps, Scene};
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use murali::frontend::{DirtyFlags, TattvaId};

const COLS: usize = 30;
const ROWS: usize = 18;
const SWAY_DURATION: f32 = 24.0;
const BALL_RADIUS: f32 = 0.22;
const BALL_AREA_MIN: Vec2 = vec2(-6.7, -3.35);
const BALL_AREA_MAX: Vec2 = vec2(6.7, 2.45);
const BALL_VELOCITY: Vec2 = vec2(2.9, 2.15);
const PULL_RADIUS: f32 = 2.2;
const MAX_PULL: f32 = 10.5;
const AIR_DRAG: f32 = 0.95;

#[derive(Clone)]
struct PointSpec {
    id: TattvaId,
    base: Vec2,
}

#[derive(Clone, Copy)]
struct PointState {
    offset: Vec2,
    velocity: Vec2,
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Swaying Points", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new(
            "A colorful point field sways while a bouncing ball pulls only the nearby points.",
            0.18,
        )
        .with_color(GRAY_B),
        3.15 * UP,
    );

    // Content
    let points = build_points(&mut scene);
    let ball_id = scene.add_tattva(
        Circle::new(BALL_RADIUS, 28, GOLD_A)
            .with_stroke(0.03, ORANGE_D),
        vec3(0.0, -0.4, 0.0),
    );

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene.camera_mut().set_view_width(16.0);

    // Timeline
    let mut timeline = Timeline::new();
    timeline
        .animate(ball_id)
        .at(0.0)
        .for_duration(0.7)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();

    let sway_points = points.clone();
    let mut point_states = vec![
        PointState {
            offset: Vec2::ZERO,
            velocity: Vec2::ZERO,
        };
        sway_points.len()
    ];
    let mut previous_elapsed = 0.0_f32;
    timeline.call_during(0.0, SWAY_DURATION, move |scene, t| {
        let elapsed = t * SWAY_DURATION;
        let dt = (elapsed - previous_elapsed).clamp(0.0, 0.05);
        previous_elapsed = elapsed;
        let ball = ball_position(elapsed);

        if let Some(ball_circle) = scene.get_tattva_typed_mut::<Circle>(ball_id) {
            let mut props = DrawableProps::write(&ball_circle.props);
            props.position = vec3(ball.x, ball.y, 0.0);
            drop(props);
            ball_circle.mark_dirty(DirtyFlags::TRANSFORM);
        }

        for (index, point) in sway_points.iter().enumerate() {
            if let Some(circle) = scene.get_tattva_typed_mut::<Circle>(point.id) {
                let state = &mut point_states[index];
                let position = point.base + state.offset;
                let attraction = ball_pull_acceleration(position, ball);

                state.velocity += attraction * dt;
                state.velocity *= AIR_DRAG.powf(dt * 60.0);
                state.velocity = state.velocity.clamp_length_max(2.4);
                state.offset += state.velocity * dt;

                let position = point.base + state.offset;

                let mut props = DrawableProps::write(&circle.props);
                props.position = vec3(position.x, position.y, 0.0);
                drop(props);

                circle.mark_dirty(DirtyFlags::TRANSFORM);
            }
        }
    });

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}

fn build_points(scene: &mut Scene) -> Vec<PointSpec> {
    let mut points = Vec::with_capacity(COLS * ROWS);
    let x_span = 12.8;
    let y_span = 7.1;

    for row in 0..ROWS {
        for col in 0..COLS {
            let tx = if COLS > 1 {
                col as f32 / (COLS - 1) as f32
            } else {
                0.0
            };
            let ty = if ROWS > 1 {
                row as f32 / (ROWS - 1) as f32
            } else {
                0.0
            };

            let base = vec2(
                lerp(-x_span * 0.5, x_span * 0.5, tx),
                lerp(-y_span * 0.5, y_span * 0.5, ty),
            );

            let index = row * COLS + col;
            let seed = index as f32;
            let radius = lerp(0.018, 0.05, hash01(seed * 5.31 + 3.8));
            let color = point_color(tx, ty, seed);

            let id = scene.add_tattva(
                Circle::new(radius, 18, color).with_stroke(0.012, color.lerp(Vec4::ONE, 0.22)),
                vec3(base.x, base.y, 0.0),
            );

            points.push(PointSpec { id, base });
        }
    }

    points
}

fn point_color(tx: f32, ty: f32, seed: f32) -> Vec4 {
    let vibgyor = [
        Vec4::new(PURPLE_D.x, PURPLE_D.y, PURPLE_D.z, 0.92), // violet
        Vec4::new(BLUE_D.x, BLUE_D.y, BLUE_D.z, 0.92), // indigo
        Vec4::new(BLUE_A.x, BLUE_A.y, BLUE_A.z, 0.92), // blue
        Vec4::new(GREEN_B.x, GREEN_B.y, GREEN_B.z, 0.92), // green
        Vec4::new(GOLD_B.x, GOLD_B.y, GOLD_B.z, 0.92), // yellow
        Vec4::new(ORANGE_D.x, ORANGE_D.y, ORANGE_D.z, 0.92), // orange
        Vec4::new(RED_B.x, RED_B.y, RED_B.z, 0.92), // red
    ];
    let spectrum_t = (tx * 0.72 + ty * 0.18 + 0.10 * hash01(seed * 2.3 + 1.1)).fract();
    let scaled = spectrum_t * (vibgyor.len() - 1) as f32;
    let idx0 = scaled.floor() as usize;
    let idx1 = (idx0 + 1).min(vibgyor.len() - 1);
    let base = vibgyor[idx0].lerp(vibgyor[idx1], scaled.fract());
    let sparkle = 0.10 + 0.18 * hash01(seed * 7.11 + 1.3);
    base.lerp(Vec4::new(1.0, 1.0, 1.0, base.w), sparkle.clamp(0.0, 0.24))
}

fn hash01(x: f32) -> f32 {
    (x.sin() * 43_758.547).fract().abs()
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ball_position(elapsed: f32) -> Vec2 {
    vec2(
        ping_pong(
            BALL_AREA_MIN.x + BALL_RADIUS,
            BALL_AREA_MAX.x - BALL_RADIUS,
            elapsed * BALL_VELOCITY.x + 2.1,
        ),
        ping_pong(
            BALL_AREA_MIN.y + BALL_RADIUS,
            BALL_AREA_MAX.y - BALL_RADIUS,
            elapsed * BALL_VELOCITY.y + 0.7,
        ),
    )
}

fn ping_pong(min: f32, max: f32, value: f32) -> f32 {
    let span = (max - min).max(0.001);
    let wrapped = value.rem_euclid(span * 2.0);
    if wrapped <= span {
        min + wrapped
    } else {
        max - (wrapped - span)
    }
}

fn ball_pull_acceleration(point: Vec2, ball: Vec2) -> Vec2 {
    let delta = ball - point;
    let distance = delta.length();
    if distance >= PULL_RADIUS || distance <= 0.0001 {
        return Vec2::ZERO;
    }

    let falloff = 1.0 - distance / PULL_RADIUS;
    let strength = MAX_PULL * falloff * falloff;
    delta.normalize() * strength
}
