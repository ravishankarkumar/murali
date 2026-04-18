use glam::{Vec3, Vec4};
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

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Neural Signal Flow", 0.34).with_color(WHITE),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    let diagram = NeuralNetworkDiagram::new(vec![3, 5, 4, 2])
        .deactivate_node(1, 2)
        .deactivate_node(2, 3);
    let flow_paths = diagram.all_path_points();

    let network_id = scene.add_tattva(diagram, Vec3::new(0.0, 0.4, 0.0));

    let signal_id = scene.add_tattva(
        SignalFlow::from_paths(flow_paths)
            .with_progress(0.0)
            .with_edge_color(Vec4::new(GOLD_C.x, GOLD_C.y, GOLD_C.z, 0.95))
            .with_pulse_color(GOLD_A),
        Vec3::new(0.0, 0.4, 0.0),
    );

    scene.add_tattva(
        Label::new(
            "All active paths propagate together; deactivated nodes stop outgoing flow.",
            0.22,
        )
        .with_color(GRAY_B),
        3.0 * DOWN,
    );

    let mut timeline = Timeline::new();
    timeline.play_signal(
        signal_id,
        SignalPlayback::round_trip(0.2, 1.8, Ease::InOutQuad),
    );

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 9.5);

    let _ = network_id;
    App::new()?.with_scene(scene).run_app()
}
