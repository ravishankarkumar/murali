use glam::{Vec2, Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::{Bounded, Bounds, Direction};
use murali::projection::{Project, ProjectionCtx, RenderPrimitive};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, SQRT_2};

const MAP_HALF_WIDTH: f32 = 5.9;
const MAP_HALF_HEIGHT: f32 = 3.6;
const TRANSITION_DURATION: f32 = 6.0;
const GRATICULE_COLOR: Vec4 = GRAY_A;
const LAND_COLOR: Vec4 = WHITE;

#[derive(Debug, Clone, Copy)]
enum ProjectionKind {
    Equirectangular,
    Sinusoidal,
    Mollweide,
    Hammer,
    Mercator,
}

impl ProjectionKind {
    fn label(self) -> &'static str {
        match self {
            ProjectionKind::Equirectangular => "Equirectangular",
            ProjectionKind::Sinusoidal => "Sinusoidal",
            ProjectionKind::Mollweide => "Mollweide",
            ProjectionKind::Hammer => "Hammer",
            ProjectionKind::Mercator => "Mercator",
        }
    }
}

#[derive(Debug, Clone)]
struct MapProjectionMorph {
    from: ProjectionKind,
    to: ProjectionKind,
    mix: f32,
}

impl MapProjectionMorph {
    fn new(initial: ProjectionKind) -> Self {
        Self {
            from: initial,
            to: initial,
            mix: 0.0,
        }
    }

    fn project_lon_lat(&self, lon: f32, lat: f32) -> Vec2 {
        let start = project_point(self.from, lon, lat);
        let end = project_point(self.to, lon, lat);
        start.lerp(end, self.mix.clamp(0.0, 1.0))
    }

    fn emit_polyline(
        &self,
        ctx: &mut ProjectionCtx,
        points: &[(f32, f32)],
        color: Vec4,
        thickness: f32,
    ) {
        for pair in points.windows(2) {
            let a = self.project_lon_lat(pair[0].0, pair[0].1);
            let b = self.project_lon_lat(pair[1].0, pair[1].1);
            ctx.emit(RenderPrimitive::Line {
                start: Vec3::new(a.x, a.y, 0.0),
                end: Vec3::new(b.x, b.y, 0.0),
                thickness,
                color,
                dash_length: 0.0,
                gap_length: 0.0,
                dash_offset: 0.0,
            });
        }
    }
}

impl Project for MapProjectionMorph {
    fn project(&self, ctx: &mut ProjectionCtx) {
        for latitude in [-60.0_f32, -30.0, 0.0, 30.0, 60.0] {
            let mut line = Vec::with_capacity(161);
            for step in 0..=160 {
                let lon = -180.0 + step as f32 * 360.0 / 160.0;
                line.push((lon.to_radians(), latitude.to_radians()));
            }
            self.emit_polyline(ctx, &line, GRATICULE_COLOR, 0.02);
        }

        for longitude in [-150.0_f32, -90.0, -30.0, 30.0, 90.0, 150.0] {
            let mut line = Vec::with_capacity(121);
            for step in 0..=120 {
                let lat = -85.0 + step as f32 * 170.0 / 120.0;
                line.push((longitude.to_radians(), lat.to_radians()));
            }
            self.emit_polyline(ctx, &line, GRATICULE_COLOR, 0.02);
        }

        for border in stylized_land_borders() {
            self.emit_polyline(ctx, &border, LAND_COLOR, 0.04);
        }
    }
}

impl Bounded for MapProjectionMorph {
    fn local_bounds(&self) -> Bounds {
        Bounds::new(
            vec2(-MAP_HALF_WIDTH - 0.4, -MAP_HALF_HEIGHT - 0.4),
            vec2(MAP_HALF_WIDTH + 0.4, MAP_HALF_HEIGHT + 0.4),
        )
    }
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Projection Morph Showcase", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new(
            "Procedural graticule and stylized coastlines morph through five world projections.",
            0.18,
        )
        .with_color(GRAY_B),
        3.15 * UP,
    );

    // Content
    let sequence = [
        ProjectionKind::Equirectangular,
        ProjectionKind::Sinusoidal,
        ProjectionKind::Mollweide,
        ProjectionKind::Hammer,
        ProjectionKind::Mercator,
        ProjectionKind::Equirectangular,
    ];

    let map_id = scene.add_tattva(
        MapProjectionMorph::new(sequence[0]),
        Vec3::new(0.0, -0.15, 0.0),
    );

    let label_id = scene.add_tattva(
        Label::new("Equirectangular -> Sinusoidal", 0.24)
            .with_color(GOLD_C),
        Vec3::new(0.0, -3.85, 0.0),
    );

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene.camera_mut().set_view_width(15.0);

    // Timeline
    let mut timeline = Timeline::new();
    for (index, pair) in sequence.windows(2).enumerate() {
        let from = pair[0];
        let to = pair[1];
        let start_time = index as f32 * TRANSITION_DURATION;

        timeline.call_at(start_time, move |scene| {
            if let Some(map) = scene.get_tattva_typed_mut::<MapProjectionMorph>(map_id) {
                map.state.from = from;
                map.state.to = to;
                map.state.mix = 0.0;
                map.mark_dirty(murali::frontend::DirtyFlags::GEOMETRY);
            }

            if let Some(label) = scene.get_tattva_typed_mut::<Label>(label_id) {
                label.state.text = format!("{} -> {}", from.label(), to.label());
                label.mark_dirty(murali::frontend::DirtyFlags::REBUILD);
            }
        });

        timeline.call_during(start_time, TRANSITION_DURATION, move |scene, t| {
            if let Some(map) = scene.get_tattva_typed_mut::<MapProjectionMorph>(map_id) {
                map.state.from = from;
                map.state.to = to;
                map.state.mix = ease_in_out_cubic(t);
                map.mark_dirty(murali::frontend::DirtyFlags::GEOMETRY);
            }
        });
    }

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}

fn project_point(kind: ProjectionKind, lon: f32, lat: f32) -> Vec2 {
    let (x, y) = match kind {
        ProjectionKind::Equirectangular => (lon / PI, lat / FRAC_PI_2),
        ProjectionKind::Sinusoidal => ((lon * lat.cos()) / PI, lat / FRAC_PI_2),
        ProjectionKind::Mollweide => {
            let theta = solve_mollweide_theta(lat);
            ((lon * theta.cos()) / PI, theta.sin())
        }
        ProjectionKind::Hammer => {
            let denom = (1.0 + lat.cos() * (lon * 0.5).cos()).sqrt().max(1e-4);
            let x = (2.0 * SQRT_2 * lat.cos() * (lon * 0.5).sin()) / denom;
            let y = (SQRT_2 * lat.sin()) / denom;
            (x / (2.0 * SQRT_2), y / SQRT_2)
        }
        ProjectionKind::Mercator => {
            let lat = lat.clamp((-80.0_f32).to_radians(), 80.0_f32.to_radians());
            let y = (FRAC_PI_4 + lat * 0.5).tan().ln();
            let y_max = (FRAC_PI_4 + 80.0_f32.to_radians() * 0.5).tan().ln();
            (lon / PI, y / y_max)
        }
    };

    vec2(x * MAP_HALF_WIDTH, y * MAP_HALF_HEIGHT)
}

fn solve_mollweide_theta(lat: f32) -> f32 {
    if (FRAC_PI_2 - lat.abs()) < 1e-4 {
        return lat.signum() * FRAC_PI_2;
    }

    let mut theta = lat;
    for _ in 0..8 {
        let numerator = 2.0 * theta + (2.0 * theta).sin() - PI * lat.sin();
        let denominator = 2.0 + 2.0 * (2.0 * theta).cos();
        theta -= numerator / denominator.max(1e-4);
    }
    theta
}

fn stylized_land_borders() -> Vec<Vec<(f32, f32)>> {
    vec![
        deg_points(&[
            (-168.0, 72.0),
            (-145.0, 70.0),
            (-128.0, 58.0),
            (-108.0, 50.0),
            (-95.0, 32.0),
            (-102.0, 16.0),
            (-118.0, 20.0),
            (-126.0, 35.0),
            (-140.0, 55.0),
            (-168.0, 72.0),
        ]),
        deg_points(&[
            (-82.0, 12.0),
            (-73.0, 8.0),
            (-69.0, -4.0),
            (-64.0, -18.0),
            (-60.0, -34.0),
            (-53.0, -53.0),
            (-42.0, -54.0),
            (-36.0, -34.0),
            (-44.0, -12.0),
            (-60.0, 6.0),
            (-82.0, 12.0),
        ]),
        deg_points(&[
            (-11.0, 36.0),
            (0.0, 50.0),
            (25.0, 58.0),
            (50.0, 60.0),
            (82.0, 55.0),
            (112.0, 54.0),
            (146.0, 58.0),
            (168.0, 48.0),
            (150.0, 26.0),
            (122.0, 15.0),
            (100.0, 8.0),
            (86.0, 20.0),
            (72.0, 25.0),
            (57.0, 25.0),
            (40.0, 34.0),
            (29.0, 31.0),
            (21.0, 12.0),
            (31.0, -10.0),
            (22.0, -31.0),
            (10.0, -35.0),
            (-4.0, -22.0),
            (-13.0, 0.0),
            (-11.0, 36.0),
        ]),
        deg_points(&[
            (112.0, -12.0),
            (131.0, -10.0),
            (149.0, -18.0),
            (154.0, -33.0),
            (145.0, -42.0),
            (124.0, -40.0),
            (113.0, -28.0),
            (112.0, -12.0),
        ]),
        deg_points(&[
            (-52.0, 82.0),
            (-35.0, 78.0),
            (-24.0, 68.0),
            (-34.0, 60.0),
            (-48.0, 62.0),
            (-58.0, 71.0),
            (-52.0, 82.0),
        ]),
    ]
}

fn deg_points(points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    points
        .iter()
        .map(|&(lon, lat)| (lon.to_radians(), lat.to_radians()))
        .collect()
}

fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) * 0.5
    }
}
