use glam::{vec2, Vec2, Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::ai::{
    attention_matrix::AttentionMatrix,
    decision_boundary_plot::{DecisionBoundaryPlot, LabeledPoint},
    neural_network_diagram::NeuralNetworkDiagram,
    token_sequence::TokenSequence,
    transformer_block_diagram::TransformerBlockDiagram,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use murali::frontend::Tattva;
use murali::App;

fn add_tattva<T>(scene: &mut Scene, state: T, position: Vec3) -> usize
where
    T: murali::projection::Project + murali::frontend::layout::Bounded + Send + Sync + 'static,
{
    let tattva = Tattva::new(0, state);
    let id = scene.add(tattva);

    if let Some(t) = scene.get_tattva_any_mut(id) {
        let mut props = t.props().write();
        props.position = position;
    }

    id
}

fn linear_classifier(p: Vec2) -> f32 {
    p.x * 0.8 - p.y * 1.1 + 0.2
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    add_tattva(
        &mut scene,
        Label::new("AI Teaching Showcase", 0.38).with_color(Vec4::new(0.97, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.3);

    add_tattva(
        &mut scene,
        NeuralNetworkDiagram::new(vec![3, 5, 4, 2]),
        Vec3::new(-4.8, 1.2, 0.0),
    );

    add_tattva(
        &mut scene,
        TransformerBlockDiagram::new(),
        Vec3::new(-0.4, 0.7, 0.0),
    );

    add_tattva(
        &mut scene,
        TokenSequence::new(vec!["The", "model", "attends", "to", "tokens"], 0.22),
        Vec3::new(0.0, -2.7, 0.0),
    );

    add_tattva(
        &mut scene,
        AttentionMatrix::new(
            vec![
                vec![0.92, 0.04, 0.02, 0.01, 0.01],
                vec![0.08, 0.70, 0.12, 0.05, 0.05],
                vec![0.05, 0.12, 0.62, 0.14, 0.07],
                vec![0.03, 0.08, 0.16, 0.58, 0.15],
                vec![0.02, 0.06, 0.11, 0.19, 0.62],
            ],
            Some(vec![
                "The".into(),
                "model".into(),
                "attends".into(),
                "to".into(),
                "tokens".into(),
            ]),
        ),
        Vec3::new(4.4, 1.0, 0.0),
    );

    let mut boundary = DecisionBoundaryPlot::new((-2.8, 2.8), (-2.2, 2.2), linear_classifier);
    boundary.points = vec![
        LabeledPoint { point: vec2(-1.8, 1.2), class: 1 },
        LabeledPoint { point: vec2(-1.2, 0.8), class: 1 },
        LabeledPoint { point: vec2(-0.9, 1.5), class: 1 },
        LabeledPoint { point: vec2(1.1, -0.7), class: 0 },
        LabeledPoint { point: vec2(1.6, -1.2), class: 0 },
        LabeledPoint { point: vec2(0.9, -0.2), class: 0 },
    ];
    boundary.grid_resolution = 20;
    add_tattva(&mut scene, boundary, Vec3::new(4.4, -2.0, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.5);

    App::new()?.with_scene(scene).run_app()
}
