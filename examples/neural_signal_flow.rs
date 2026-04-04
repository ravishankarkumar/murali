use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::{
    neural_network_diagram::NeuralNetworkDiagram, signal_flow::SignalFlow,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Neural Signal Flow", 0.34).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    let diagram = NeuralNetworkDiagram::new(vec![3, 5, 4, 2]);
    let flow_points = diagram
        .path_points(&[1, 3, 1, 0])
        .expect("signal flow path should match layer count");

    let network_id = scene.add_tattva(diagram, Vec3::new(0.0, 0.4, 0.0));

    let signal_id = scene.add_tattva(
        SignalFlow::new(flow_points)
            .with_progress(0.0)
            .with_edge_color(Vec4::new(0.98, 0.74, 0.28, 0.95))
            .with_pulse_color(Vec4::new(1.0, 0.96, 0.82, 1.0)),
        Vec3::new(0.0, 0.4, 0.0),
    );

    scene.add_tattva(
        Label::new("A pulse travels along one selected activation path.", 0.22)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.0, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(signal_id)
        .at(0.2)
        .for_duration(1.8)
        .ease(Ease::InOutQuad)
        .propagate()
        .spawn();

    timeline
        .animate(signal_id)
        .at(2.4)
        .for_duration(1.6)
        .ease(Ease::InOutQuad)
        .propagate_to(0.0)
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 9.5);

    let _ = network_id;
    App::new()?.with_scene(scene).run_app()
}
