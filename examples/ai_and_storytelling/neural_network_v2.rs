use glam::{Vec3};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::{SignalPlayback, Timeline};
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::{
    neural_network_diagram::NeuralNetworkDiagram, signal_flow::SignalFlow,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn add_network_panel(
    scene: &mut Scene,
    timeline: &mut Timeline,
    title: &str,
    caption: &str,
    origin: Vec3,
    playback: SignalPlayback,
) {
    scene.add_tattva(
        Label::new(title, 0.26).with_color(WHITE),
        origin + Vec3::new(0.0, 2.35, 0.0),
    );

    let diagram = NeuralNetworkDiagram::new(vec![3, 4, 3, 2])
        .deactivate_node(1, 2)
        .deactivate_node(2, 1);
    let flow_paths = diagram.all_path_points();

    scene.add_tattva(diagram, origin);

    let signal_id = scene.add_tattva(
        SignalFlow::from_paths(flow_paths)
            .with_progress(0.0)
            .with_edge_color(GOLD_C)
            .with_pulse_color(GOLD_A),
        origin,
    );

    scene.add_tattva(
        Label::new(caption, 0.18).with_color(GRAY_A),
        origin + Vec3::new(0.0, -2.35, 0.0),
    );

    timeline.play_signal(signal_id, playback);
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Neural Network V2", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.6);

    scene.add_tattva(
        Label::new(
            "Incoming edges to deactivated nodes stay normal. Outgoing edges dim, and flow stops there.",
            0.20,
        )
        .with_color(GRAY_A),
        3.0 * UP,
    );

    let mut timeline = Timeline::new();

    add_network_panel(
        &mut scene,
        &mut timeline,
        "Once",
        "Single forward pass through all valid paths.",
        Vec3::new(-5.4, -0.1, 0.0),
        SignalPlayback::once(0.2, 1.8, Ease::InOutQuad),
    );

    add_network_panel(
        &mut scene,
        &mut timeline,
        "Round Trip",
        "Forward, then back to the start.",
        Vec3::new(0.0, -0.1, 0.0),
        SignalPlayback::round_trip(0.2, 1.6, Ease::InOutQuad),
    );

    add_network_panel(
        &mut scene,
        &mut timeline,
        "Loop x3",
        "Repeats the forward pass three times.",
        Vec3::new(5.4, -0.1, 0.0),
        SignalPlayback::looped(0.2, 1.1, 3, Ease::Linear),
    );

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 13.5);

    App::new()?.with_scene(scene).run_app()
}
