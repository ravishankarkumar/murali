/// Agentic Flow Chart with Write Animation and Progressive Edges
/// Demonstrates nodes drawing themselves with write effect
/// and edges appearing progressively as nodes are revealed
use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::agentic_flow_chart::{
    AgenticFlowChart, EdgeStep, FlowChartDirection, FlowEdge, FlowNode, FlowNodeShape,
    NodeAnimationStyle,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("Agentic Loop: Write Animation + Progressive Edges", 0.34)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    let nodes = vec![
        FlowNode::new("Observe")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(Vec4::new(0.16, 0.36, 0.31, 1.0))
            .with_stroke_color(Vec4::new(0.44, 0.84, 0.71, 1.0)),
        FlowNode::new("Reason")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(Vec4::new(0.19, 0.22, 0.35, 1.0))
            .with_stroke_color(Vec4::new(0.44, 0.62, 0.95, 1.0)),
        FlowNode::new("Plan")
            .with_shape(FlowNodeShape::Diamond)
            .with_size(vec2(2.2, 1.2))
            .with_fill_color(Vec4::new(0.33, 0.22, 0.15, 1.0))
            .with_stroke_color(Vec4::new(0.98, 0.72, 0.34, 1.0)),
        FlowNode::new("Act")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(Vec4::new(0.20, 0.24, 0.30, 1.0))
            .with_stroke_color(Vec4::new(0.78, 0.82, 0.90, 1.0)),
        FlowNode::new("Reflect")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(Vec4::new(0.30, 0.17, 0.17, 1.0))
            .with_stroke_color(Vec4::new(0.94, 0.48, 0.44, 1.0)),
    ];

    let chart = AgenticFlowChart::new(nodes)
        .with_direction(FlowChartDirection::Horizontal)
        .with_edges(vec![
            FlowEdge::new(0, 1),
            FlowEdge::new(1, 2),
            FlowEdge::new(2, 3),
            FlowEdge::new(3, 1).with_route_steps(vec![EdgeStep::Up, EdgeStep::Left, EdgeStep::Left, EdgeStep::Down]),
            FlowEdge::new(3, 4),
        ])
        .with_flow_path(vec![0, 1, 2, 3, 1, 2, 3, 4])
        .with_reveal_progress(0.0)
        .with_progress(0.0)
        .with_node_gap(0.95)
        .with_lane_gap(0.85)
        .with_active_edge_color(Vec4::new(0.98, 0.80, 0.30, 1.0))
        .with_pulse_color(Vec4::new(1.0, 0.96, 0.84, 1.0))
        // Enable write animation for nodes
        .with_node_animation_style(NodeAnimationStyle::Write)
        // Enable progressive edge drawing
        .with_progressive_edges(true);

    let chart_id = scene.add_tattva(chart, Vec3::new(0.0, 0.25, 0.0));

    scene.add_tattva(
        Label::new("Nodes draw with write effect, edges appear progressively", 0.22)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.1, 0.0),
    );

    let mut timeline = Timeline::new();

    // Phase 1: Sequential reveal with write animation
    // Each node draws itself progressively
    let reveal_start = 0.3;
    let reveal_duration = 5.0;  // Increased from 3.5 to give more time for write animation
    
    timeline
        .animate(chart_id)
        .at(reveal_start)
        .for_duration(reveal_duration)
        .ease(Ease::Linear)
        .reveal_to(1.0)
        .spawn();

    // Phase 2: Flow propagation through the path
    // Starts after reveal is complete
    let flow_start = reveal_start + reveal_duration + 0.3;
    let flow_duration = 5.6;
    
    timeline
        .animate(chart_id)
        .at(flow_start)
        .for_duration(flow_duration)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
