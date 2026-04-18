use glam::{Quat, Vec2, Vec3, Vec4, vec2, vec3};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::{DrawableProps, Scene};
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::rectangle::Rectangle;
use murali::frontend::collection::text::label::Label;
use murali::frontend::style::ColorSource;
use murali::frontend::{DirtyFlags, TattvaId};
use murali::projection::style::StrokeParams;

const SHAPE_COUNT: usize = 420;
const REVEAL_START: f32 = 0.5;
const REVEAL_LAG: f32 = 0.003;
const REVEAL_DURATION: f32 = 0.55;
const MORPH_START: f32 = 2.0;
const MORPH_DURATION: f32 = 3.0;
const DROP_START: f32 = 5.6;
const DROP_DURATION: f32 = 1.25;
const DROP_LAG: f32 = 0.0015;

#[derive(Clone)]
enum ShapeKind {
    Circle {
        from_radius: f32,
        to_radius: f32,
    },
    Rect {
        from_size: Vec2,
        to_size: Vec2,
        from_angle: f32,
        to_angle: f32,
    },
}

#[derive(Clone)]
struct ShapeSpec {
    id: TattvaId,
    from_position: Vec2,
    to_position: Vec2,
    from_color: Vec4,
    to_color: Vec4,
    kind: ShapeKind,
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    // let title_id = scene.add_tattva(
    //     Label::new("Shape Storm", 0.36).with_color(WHITE),
    //     Vec3::ZERO,
    // );
    // scene.to_edge(title_id, murali::frontend::layout::Direction::Up, 0.35);

    // // Subtitle
    // scene.add_tattva(
    //     Label::new(
    //         "Hundreds of circles and rectangles appear, remix themselves, then tumble out together.",
    //         0.18,
    //     )
    //     .with_color(GRAY_B),
    //     3.15 * UP,
    // );

    // Content
    let shapes = build_shapes(&mut scene);

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene.camera_mut().set_view_width(16.0);

    // Timeline
    let mut timeline = Timeline::new();

    for (index, shape) in shapes.iter().enumerate() {
        let reveal_at = REVEAL_START + index as f32 * REVEAL_LAG;
        timeline
            .animate(shape.id)
            .at(reveal_at)
            .for_duration(REVEAL_DURATION)
            .ease(Ease::OutCubic)
            .appear()
            .spawn();

        timeline
            .animate(shape.id)
            .at(reveal_at)
            .for_duration(REVEAL_DURATION)
            .ease(Ease::OutCubic)
            .scale_to(Vec3::ONE)
            .spawn();
    }

    let morph_shapes = shapes.clone();
    timeline.call_during(MORPH_START, MORPH_DURATION, move |scene, t| {
        let eased = ease_in_out_quint(t);
        for shape in &morph_shapes {
            let position = shape.from_position.lerp(shape.to_position, eased);
            let color = shape.from_color.lerp(shape.to_color, eased);
            apply_shape(scene, shape, position, color, eased);
        }
    });

    let drop_shapes = shapes.clone();
    timeline.call_during(
        DROP_START,
        DROP_DURATION + SHAPE_COUNT as f32 * DROP_LAG,
        move |scene, t| {
            let overall_time = t * (DROP_DURATION + SHAPE_COUNT as f32 * DROP_LAG);
            let drop_floor = scene.frame_bounds().min.y - 1.2;

            for (index, shape) in drop_shapes.iter().enumerate() {
                let local_time = (overall_time - index as f32 * DROP_LAG).clamp(0.0, DROP_DURATION);
                let local_t = (local_time / DROP_DURATION).clamp(0.0, 1.0);
                let eased = ease_out_bounce(local_t);

                let start = shape.to_position;
                let end = vec2(start.x, drop_floor);
                let position = start.lerp(end, eased);
                set_shape_position(scene, shape.id, position);
            }
        },
    );

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}

fn build_shapes(scene: &mut Scene) -> Vec<ShapeSpec> {
    let mut shapes = Vec::with_capacity(SHAPE_COUNT);

    for index in 0..SHAPE_COUNT {
        let circle = hash01(index as f32 * 13.17 + 0.23) > 0.5;
        let from_position = random_position(index, 0.0);
        let to_position = random_position(index, 17.0);
        let from_color = random_color(index, 1.0);
        let to_color = random_color(index, 21.0);

        if circle {
            let from_radius = random_radius(index, 2.0);
            let to_radius = random_radius(index, 11.0);

            let id = scene.add_tattva(
                Circle::new(from_radius, 24, from_color)
                    .with_stroke(0.024, stroke_color(from_color)),
                vec3(from_position.x, from_position.y, 0.0),
            );
            scene.hide(id);
            scene.set_scale(id, vec3(0.25, 0.25, 1.0));

            shapes.push(ShapeSpec {
                id,
                from_position,
                to_position,
                from_color,
                to_color,
                kind: ShapeKind::Circle {
                    from_radius,
                    to_radius,
                },
            });
        } else {
            let from_size = random_size(index, 4.0);
            let to_size = random_size(index, 14.0);
            let from_angle = random_angle(index, 5.0);
            let to_angle = random_angle(index, 24.0);

            let id = scene.add_tattva(
                Rectangle::new(from_size.x, from_size.y, from_color)
                    .with_stroke(0.024, stroke_color(from_color)),
                vec3(from_position.x, from_position.y, 0.0),
            );
            scene.hide(id);
            scene.set_scale(id, vec3(0.25, 0.25, 1.0));
            scene.set_rotation(id, Quat::from_rotation_z(from_angle));

            shapes.push(ShapeSpec {
                id,
                from_position,
                to_position,
                from_color,
                to_color,
                kind: ShapeKind::Rect {
                    from_size,
                    to_size,
                    from_angle,
                    to_angle,
                },
            });
        }
    }

    shapes
}

fn apply_shape(scene: &mut Scene, shape: &ShapeSpec, position: Vec2, color: Vec4, t: f32) {
    match &shape.kind {
        ShapeKind::Circle {
            from_radius,
            to_radius,
        } => {
            if let Some(circle) = scene.get_tattva_typed_mut::<Circle>(shape.id) {
                circle.state.radius = lerp(*from_radius, *to_radius, t);
                circle.state.style.fill = Some(ColorSource::Solid(color));
                circle.state.style.stroke = Some(StrokeParams {
                    thickness: 0.024,
                    color: stroke_color(color),
                    ..Default::default()
                });

                let mut props = DrawableProps::write(&circle.props);
                props.position = vec3(position.x, position.y, 0.0);
                props.rotation = Quat::IDENTITY;
                drop(props);

                circle.mark_dirty(
                    DirtyFlags::GEOMETRY
                        | DirtyFlags::STYLE
                        | DirtyFlags::BOUNDS
                        | DirtyFlags::TRANSFORM,
                );
            }
        }
        ShapeKind::Rect {
            from_size,
            to_size,
            from_angle,
            to_angle,
        } => {
            if let Some(rect) = scene.get_tattva_typed_mut::<Rectangle>(shape.id) {
                let size = from_size.lerp(*to_size, t);
                rect.state.width = size.x;
                rect.state.height = size.y;
                rect.state.style.fill = Some(ColorSource::Solid(color));
                rect.state.style.stroke = Some(StrokeParams {
                    thickness: 0.024,
                    color: stroke_color(color),
                    ..Default::default()
                });

                let mut props = DrawableProps::write(&rect.props);
                props.position = vec3(position.x, position.y, 0.0);
                props.rotation = Quat::from_rotation_z(lerp(*from_angle, *to_angle, t));
                drop(props);

                rect.mark_dirty(
                    DirtyFlags::GEOMETRY
                        | DirtyFlags::STYLE
                        | DirtyFlags::BOUNDS
                        | DirtyFlags::TRANSFORM,
                );
            }
        }
    }
}

fn set_shape_position(scene: &mut Scene, id: TattvaId, position: Vec2) {
    if let Some(circle) = scene.get_tattva_typed_mut::<Circle>(id) {
        let mut props = DrawableProps::write(&circle.props);
        props.position = vec3(position.x, position.y, 0.0);
        drop(props);
        circle.mark_dirty(DirtyFlags::TRANSFORM);
        return;
    }

    if let Some(rect) = scene.get_tattva_typed_mut::<Rectangle>(id) {
        let mut props = DrawableProps::write(&rect.props);
        props.position = vec3(position.x, position.y, 0.0);
        drop(props);
        rect.mark_dirty(DirtyFlags::TRANSFORM);
    }
}

fn random_position(index: usize, salt: f32) -> Vec2 {
    let x = lerp(-6.8, 6.8, hash01(index as f32 * 7.31 + salt));
    let y = lerp(-2.9, 2.6, hash01(index as f32 * 11.17 + salt + 4.2));
    vec2(x, y)
}

fn random_radius(index: usize, salt: f32) -> f32 {
    lerp(0.10, 0.55, hash01(index as f32 * 19.1 + salt))
}

fn random_size(index: usize, salt: f32) -> Vec2 {
    let w = lerp(0.18, 1.08, hash01(index as f32 * 23.1 + salt));
    let h = lerp(0.18, 1.08, hash01(index as f32 * 29.7 + salt + 8.0));
    vec2(w, h)
}

fn random_angle(index: usize, salt: f32) -> f32 {
    lerp(
        -std::f32::consts::PI,
        std::f32::consts::PI,
        hash01(index as f32 * 31.7 + salt),
    )
}

fn random_color(index: usize, salt: f32) -> Vec4 {
    let palette = [
        Vec4::new(RED_E.x, RED_E.y, RED_E.z, 1.0),
        Vec4::new(GOLD_E.x, GOLD_E.y, GOLD_E.z, 1.0),
        Vec4::new(TEAL_E.x, TEAL_E.y, TEAL_E.z, 1.0),
        Vec4::new(BLUE_E.x, BLUE_E.y, BLUE_E.z, 1.0),
        Vec4::new(PURPLE_E.x, PURPLE_E.y, PURPLE_E.z, 1.0),
    ];
    let wrapped = hash01(index as f32 * 37.2 + salt) * palette.len() as f32;
    let idx0 = wrapped.floor() as usize % palette.len();
    let idx1 = (idx0 + 1) % palette.len();
    palette[idx0].lerp(palette[idx1], wrapped.fract())
}

fn stroke_color(fill: Vec4) -> Vec4 {
    fill.lerp(Vec4::new(1.0, 1.0, 1.0, fill.w), 0.28)
}

fn hash01(x: f32) -> f32 {
    (x.sin() * 43_758.547).fract().abs()
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn ease_in_out_quint(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        16.0 * t.powi(5)
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

fn ease_out_bounce(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}
