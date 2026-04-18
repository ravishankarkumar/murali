use glam::{Vec2, Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::path::Path;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::utility::TracedPath;
use murali::frontend::layout::{Bounded, Bounds, Direction};
use murali::frontend::style::{StrokeParams, Style};
use murali::projection::{Project, ProjectionCtx, RenderPrimitive};
use std::f32::consts::TAU;

const TRACE_DURATION: f32 = 30.0;
const CIRCLE_COLOR: Vec4 = Vec4::new(GRAY_A.x, GRAY_A.y, GRAY_A.z, 0.72);
const SPOKE_COLOR: Vec4 = Vec4::new(WHITE.x, WHITE.y, WHITE.z, 0.92);
const GUIDE_COLOR: Vec4 = Vec4::new(GREEN_C.x, GREEN_C.y, GREEN_C.z, 0.42);
const TRACE_COLOR: Vec4 = ORANGE_C;

#[derive(Debug, Clone, Copy)]
struct Complex32 {
    re: f32,
    im: f32,
}

impl Complex32 {
    const ZERO: Self = Self { re: 0.0, im: 0.0 };

    fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    fn from_polar(radius: f32, angle: f32) -> Self {
        Self::new(radius * angle.cos(), radius * angle.sin())
    }

    fn to_vec2(self) -> Vec2 {
        vec2(self.re, self.im)
    }

    fn magnitude(self) -> f32 {
        self.re.hypot(self.im)
    }

    fn rotate(self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(self.re * cos - self.im * sin, self.re * sin + self.im * cos)
    }
}

impl std::ops::Add for Complex32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::AddAssign for Complex32 {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

#[derive(Debug, Clone, Copy)]
struct FourierCoeff {
    frequency: i32,
    value: Complex32,
}

#[derive(Debug, Clone)]
struct FourierEpicycles {
    coeffs: Vec<FourierCoeff>,
    phase: f32,
    circle_thickness: f32,
    spoke_thickness: f32,
}

impl FourierEpicycles {
    fn new(coeffs: Vec<FourierCoeff>) -> Self {
        Self {
            coeffs,
            phase: 0.0,
            circle_thickness: 0.02,
            spoke_thickness: 0.028,
        }
    }

    fn tip(&self) -> Vec2 {
        let mut sum = Complex32::ZERO;
        for coeff in &self.coeffs {
            sum += coeff
                .value
                .rotate(TAU * coeff.frequency as f32 * self.phase);
        }
        sum.to_vec2()
    }

    fn radius_budget(&self) -> f32 {
        self.coeffs
            .iter()
            .map(|coeff| coeff.value.magnitude())
            .sum()
    }
}

impl Project for FourierEpicycles {
    fn project(&self, ctx: &mut ProjectionCtx) {
        let mut origin = Vec2::ZERO;
        for coeff in &self.coeffs {
            let offset = coeff
                .value
                .rotate(TAU * coeff.frequency as f32 * self.phase);
            let next = origin + offset.to_vec2();
            let radius = coeff.value.magnitude();

            if radius > 0.001 {
                let segments = 72;
                for i in 0..segments {
                    let a0 = TAU * (i as f32 / segments as f32);
                    let a1 = TAU * ((i + 1) as f32 / segments as f32);
                    let p0 = origin + vec2(radius * a0.cos(), radius * a0.sin());
                    let p1 = origin + vec2(radius * a1.cos(), radius * a1.sin());
                    ctx.emit(RenderPrimitive::Line {
                        start: Vec3::new(p0.x, p0.y, 0.0),
                        end: Vec3::new(p1.x, p1.y, 0.0),
                        thickness: self.circle_thickness,
                        color: CIRCLE_COLOR,
                        dash_length: 0.0,
                        gap_length: 0.0,
                        dash_offset: 0.0,
                    });
                }
            }

            ctx.emit(RenderPrimitive::Line {
                start: Vec3::new(origin.x, origin.y, 0.0),
                end: Vec3::new(next.x, next.y, 0.0),
                thickness: self.spoke_thickness,
                color: SPOKE_COLOR,
                dash_length: 0.0,
                gap_length: 0.0,
                dash_offset: 0.0,
            });

            origin = next;
        }
    }
}

impl Bounded for FourierEpicycles {
    fn local_bounds(&self) -> Bounds {
        let padding = self.radius_budget().max(0.5);
        Bounds::new(Vec2::splat(-padding), Vec2::splat(padding))
    }
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Simple Fourier Trace", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new(
            "Just three epicycles, slowed down to make the construction easy to watch.",
            0.18,
        )
        .with_color(GRAY_B),
        3.15 * UP,
    );

    // Content
    let coeffs = simple_coefficients();
    let guide_points = sample_outline(&coeffs, 360);
    let epicycles = FourierEpicycles::new(coeffs);
    let initial_tip = epicycles.tip();

    let guide_id = scene.add_tattva(guide_path(&guide_points), Vec3::ZERO);
    scene.set_opacity(guide_id, 0.3);

    let epicycle_id = scene.add_tattva(epicycles, Vec3::ZERO);
    let tip_id = scene.add_tattva(
        Circle::new(0.1, 28, TRACE_COLOR).with_stroke(0.024, GOLD_A),
        Vec3::new(initial_tip.x, initial_tip.y, 0.0),
    );

    let traced_path_id = scene.add_tattva(
        TracedPath::new(tip_id, |pos, _rot| pos, TRACE_COLOR, 0.045)
            .with_min_distance(0.01)
            .with_max_points(2000),
        Vec3::ZERO,
    );

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene.camera_mut().set_view_width(12.0);

    // Timeline
    let mut timeline = Timeline::new();
    timeline
        .animate(guide_id)
        .at(0.0)
        .for_duration(1.2)
        .draw()
        .spawn();

    timeline
        .animate(tip_id)
        .at(0.0)
        .for_duration(0.8)
        .appear()
        .spawn();

    timeline.call_during(0.0, TRACE_DURATION, move |scene, t| {
        if let Some(epicycle) = scene.get_tattva_typed_mut::<FourierEpicycles>(epicycle_id) {
            epicycle.state.phase = t.clamp(0.0, 1.0);
            epicycle.mark_dirty(murali::frontend::DirtyFlags::GEOMETRY);
            let tip = epicycle.state.tip();
            scene.set_position_2d(tip_id, tip);
        }
    });

    timeline.call_at(TRACE_DURATION, move |scene| {
        if let Some(traced_path) = scene.get_tattva_typed_mut::<TracedPath>(traced_path_id) {
            traced_path.state.stop_recording();
            traced_path.mark_dirty(murali::frontend::DirtyFlags::GEOMETRY);
        }
    });

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}

fn simple_coefficients() -> Vec<FourierCoeff> {
    vec![
        FourierCoeff {
            frequency: 1,
            value: Complex32::from_polar(2.2, 0.0),
        },
        FourierCoeff {
            frequency: -2,
            value: Complex32::from_polar(0.78, 0.55),
        },
        FourierCoeff {
            frequency: 3,
            value: Complex32::from_polar(0.34, -0.9),
        },
    ]
}

fn sample_outline(coeffs: &[FourierCoeff], samples: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(samples + 1);
    for index in 0..=samples {
        let phase = index as f32 / samples as f32;
        let mut sum = Complex32::ZERO;
        for coeff in coeffs {
            sum += coeff.value.rotate(TAU * coeff.frequency as f32 * phase);
        }
        points.push(sum.to_vec2());
    }
    points
}

fn guide_path(points: &[Vec2]) -> Path {
    let mut path = Path::new()
        .with_style(Style::new().with_stroke(StrokeParams {
            thickness: 0.035,
            color: GUIDE_COLOR,
            ..Default::default()
        }))
        .move_to(points[0]);

    for point in &points[1..] {
        path = path.line_to(*point);
    }

    path
}
