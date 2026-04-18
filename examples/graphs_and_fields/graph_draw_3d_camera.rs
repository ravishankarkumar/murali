use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::camera::Projection;
use murali::engine::scene::Scene;
use murali::engine::timeline::{SignalPlayback, Timeline};
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::signal_flow::SignalFlow;
use murali::frontend::collection::composite::axes3d::Axes3D;
use murali::frontend::collection::graph::parametric_curve3d::ParametricCurve3D;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn space_curve(t: f32) -> Vec3 {
    Vec3::new(
        3.2 * (0.85 * t).cos(),
        1.4 * (1.55 * t).sin(),
        -2.8 + 0.9 * t + 0.45 * (1.1 * t).cos(),
    )
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("3D Curve Drawn In Space", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.add_tattva(
        Axes3D::new((-6.0, 6.0), (-4.0, 4.0), (-4.5, 4.5))
            .with_step(1.0)
            .with_axis_thickness(0.04)
            .with_tick_size(0.18),
        Vec3::ZERO,
    );

    let graph = ParametricCurve3D::new((0.0, 6.2), space_curve).with_samples(260);
    let draw_points: Vec<Vec3> = graph.sample_points();

    let graph_id = scene.add_tattva(graph, Vec3::ZERO);

    let mut draw_flow = SignalFlow::new(draw_points)
        .with_progress(0.0)
        .with_edge_color(GOLD_C)
        .with_pulse_color(GOLD_A);
    draw_flow.highlight_nodes = false;
    draw_flow.node_radius = 0.0;
    draw_flow.edge_thickness = 0.055;
    draw_flow.pulse_radius = 0.13;

    let draw_id = scene.add_tattva(draw_flow, Vec3::ZERO);

    scene.add_tattva(
        Label::new("x(t), y(t), z(t) all evolve together", 0.24)
            .with_color(GOLD_C),
        Vec3::new(2.0, 2.85, 0.0),
    );

    scene.add_tattva(
        Label::new("x", 0.24).with_color(ORANGE_B),
        Vec3::new(6.3, 0.0, 0.0),
    );

    scene.add_tattva(
        Label::new("y", 0.24).with_color(BLUE_B),
        Vec3::new(0.0, 4.35, 0.0),
    );

    scene.add_tattva(
        Label::new("z", 0.24).with_color(GOLD_C),
        Vec3::new(0.0, 0.0, 4.9),
    );

    scene.add_tattva(
        Label::new(
            "A true 3D parametric curve is drawn while the camera moves at an oblique angle.",
            0.20,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -4.35, 0.0),
    );

    scene.camera_mut().projection = Projection::Perspective {
        fov_y_rad: 45.0_f32.to_radians(),
        aspect: 16.0 / 9.0,
        near: 0.1,
        far: 100.0,
    };
    scene.camera_mut().position = Vec3::new(-8.2, 4.4, 6.2);
    scene.camera_mut().target = Vec3::new(0.6, 0.2, 1.4);

    let mut timeline = Timeline::new();
    timeline.play_signal(draw_id, SignalPlayback::once(0.2, 4.2, Ease::InOutCubic));

    timeline
        .animate(graph_id)
        .at(0.0)
        .for_duration(0.3)
        .ease(Ease::Linear)
        .fade_to(0.28)
        .spawn();

    timeline
        .animate_camera()
        .at(0.0)
        .for_duration(2.2)
        .ease(Ease::InOutQuad)
        .frame_to(Vec3::new(-5.6, 2.9, 5.0), Vec3::new(-0.2, 0.1, 1.2))
        .spawn();

    timeline
        .animate_camera()
        .at(2.2)
        .for_duration(2.3)
        .ease(Ease::InOutQuad)
        .frame_to(Vec3::new(4.8, 2.1, 3.9), Vec3::new(0.7, -0.1, 1.5))
        .spawn();

    timeline
        .animate_camera()
        .at(4.5)
        .for_duration(1.4)
        .ease(Ease::OutQuad)
        .frame_to(Vec3::new(1.8, 6.3, 5.2), Vec3::new(0.0, 0.3, 1.4))
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
