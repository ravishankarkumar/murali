use glam::{Vec3};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::agentic_flow_chart::{
    AgenticFlowChart, EdgeStep, FlowEdge, FlowNode, FlowNodeShape,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("AI Agent Loop: Sequential Reveal", 0.35)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.4);

    let nodes = vec![
        FlowNode::new("Observe")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(GRAY_D)
            .with_reveal_at(0.0),
        FlowNode::new("Reason")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(GRAY_D)
            .with_reveal_at(0.2),
        FlowNode::new("Plan")
            .with_shape(FlowNodeShape::Diamond)
            .with_fill_color(GOLD_E),
        FlowNode::new("Act")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(GREEN_D),
        FlowNode::new("Reflect")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(MAROON_D)
            .with_reveal_at(0.85),
    ];

    let edges = vec![
        FlowEdge::new(0, 1),
        FlowEdge::new(1, 2),
        FlowEdge::new(2, 3),
        FlowEdge::new(3, 4),
        FlowEdge::new(4, 0)
            .with_route_steps(vec![
                EdgeStep::Up,
                EdgeStep::Left,
                EdgeStep::Left,
                EdgeStep::Left,
                EdgeStep::Left,
                EdgeStep::Down,
            ])
            .with_reveal_at(0.95),
    ];

    let chart = AgenticFlowChart::new(nodes)
        .with_edges(edges)
        .with_flow_path(vec![0, 1, 2, 3, 4, 0])
        .with_reveal_progress(0.0)
        .with_node_gap(1.1)
        .with_lane_gap(0.7);

    let chart_id = scene.add_tattva(chart, ORIGIN);

    let mut timeline = Timeline::new();

    // 1. Reveal stage
    timeline
        .animate(chart_id)
        .at(0.5)
        .for_duration(4.5)
        .ease(Ease::Linear)
        .reveal_to(1.0)
        .spawn();

    // 2. Pulse stage (starts after reveal is almost done)
    timeline
        .animate(chart_id)
        .at(5.5)
        .for_duration(6.0)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
