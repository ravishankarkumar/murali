/// Agentic Flow Chart with Write Animation and Progressive Edges
/// Demonstrates nodes drawing themselves with write effect
/// and edges appearing progressively as nodes are revealed
use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
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
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    let nodes = vec![
        FlowNode::new("Observe")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(GREEN_D)
            .with_stroke_color(TEAL_C),
        FlowNode::new("Reason")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(GRAY_D)
            .with_stroke_color(BLUE_B),
        FlowNode::new("Plan")
            .with_shape(FlowNodeShape::Diamond)
            .with_size(vec2(2.2, 1.2))
            .with_fill_color(GOLD_E)
            .with_stroke_color(ORANGE_D),
        FlowNode::new("Act")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(GRAY_D)
            .with_stroke_color(GRAY_A),
        FlowNode::new("Reflect")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(MAROON_D)
            .with_stroke_color(RED_B),
    ];

    let chart = AgenticFlowChart::new(nodes)
        .with_direction(FlowChartDirection::Horizontal)
        .with_edges(vec![
            FlowEdge::new(0, 1),
            FlowEdge::new(1, 2),
            FlowEdge::new(2, 3),
            FlowEdge::new(3, 1).with_route_steps(vec![
                EdgeStep::Up,
                EdgeStep::Left,
                EdgeStep::Left,
                EdgeStep::Down,
            ]),
            FlowEdge::new(3, 4),
        ])
        .with_flow_path(vec![0, 1, 2, 3, 1, 2, 3, 4])
        .with_reveal_progress(0.0)
        .with_progress(0.0)
        .with_node_gap(0.95)
        .with_lane_gap(0.85)
        .with_active_edge_color(GOLD_C)
        .with_pulse_color(GOLD_A)
        // Enable write animation for nodes
        .with_node_animation_style(NodeAnimationStyle::Write)
        // Enable progressive edge drawing
        .with_progressive_edges(true);

    // Add chart FIRST so it renders behind the labels
    let chart_id = scene.add_tattva(chart, Vec3::new(0.0, 0.25, 0.0));

    // Create Label tattvas for each node and position them at node centers
    // Labels are added AFTER the chart so they render on top
    let mut label_ids = Vec::new();

    // First, collect node data (labels, colors, positions)
    let node_data: Vec<(String, Vec4, Vec3)> = {
        let chart_ref = scene
            .get_tattva_typed::<AgenticFlowChart>(chart_id)
            .unwrap();
        chart_ref
            .state
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let node_center = chart_ref.state.node_center(i).unwrap_or(Vec3::ZERO);
                // Position labels at the same position as node centers
                let label_position = Vec3::new(0.0, 0.25, 0.0) + node_center;
                (node.label.clone(), node.text_color, label_position)
            })
            .collect()
    };

    // Now create the labels (after dropping the immutable borrow)
    for (label_text, text_color, label_position) in node_data {
        let label = Label::new(&label_text, 0.22).with_color(text_color);
        let label_id = scene.add_tattva(label, label_position);
        label_ids.push(Some(label_id));
    }

    // Update chart with label IDs
    if let Some(chart_mut) = scene.get_tattva_typed_mut::<AgenticFlowChart>(chart_id) {
        chart_mut.state.label_ids = label_ids.clone();
    }

    scene.add_tattva(
        Label::new(
            "Nodes draw with write effect, text types character by character",
            0.22,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -3.1, 0.0),
    );

    let mut timeline = Timeline::new();

    // Phase 1: Sequential reveal with write animation
    // Each node draws itself progressively
    let reveal_start = 0.3;
    let reveal_duration = 5.0; // Increased from 3.5 to give more time for write animation

    timeline
        .animate(chart_id)
        .at(reveal_start)
        .for_duration(reveal_duration)
        .ease(Ease::Linear)
        .reveal_to(1.0)
        .spawn();

    // Add typewriter animations for each label, synchronized with node reveal
    // We need to calculate when each node starts revealing
    let node_count = label_ids.len();
    let node_reveal_window = 0.2; // Same as in node_write_progress

    for (i, label_id_opt) in label_ids.iter().enumerate() {
        if let Some(label_id) = label_id_opt {
            // Calculate when this node starts revealing
            let node_threshold = i as f32 / node_count as f32;
            let node_start_time = reveal_start + (reveal_duration * node_threshold);
            let write_duration = reveal_duration * node_reveal_window;

            timeline
                .animate(*label_id)
                .at(node_start_time)
                .for_duration(write_duration)
                .ease(Ease::Linear)
                .typewrite_text()
                .spawn();
        }
    }

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

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
