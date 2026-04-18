use glam::{Vec3, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::agentic_flow_chart::{
    AgenticFlowChart, EdgeStep, FlowChartDirection, FlowEdge, FlowNode, FlowNodePlacement,
    FlowNodeShape,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("Deprecated: Agentic Loop / Flow Chart", 0.34).with_color(WHITE),
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
            .with_stroke_color(ORANGE_D)
            .with_placement(FlowNodePlacement::BelowPrevious),
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
            // FlowEdge::new(2, 1),
            FlowEdge::new(2, 3),
            FlowEdge::new(3, 1).with_route_steps(vec![EdgeStep::Up, EdgeStep::Left]),
            FlowEdge::new(3, 4),
        ])
        .with_flow_path(vec![0, 1, 2, 3, 1, 2, 3, 4])
        .with_reveal_progress(0.0)
        .with_progress(0.0)
        .with_node_gap(0.95)
        .with_lane_gap(0.85)
        .with_active_edge_color(GOLD_C)
        .with_pulse_color(GOLD_A);

    let chart_start = 0.2;
    let chart_duration = 5.6;
    let arrivals = chart.node_arrivals(chart_start, chart_duration);
    let completion_time = chart.completion_time(chart_start, chart_duration);

    let chart_id = scene.add_tattva(chart, Vec3::new(0.0, 0.25, 0.0));

    scene.add_tattva(
        Label::new("Path: 0 -> 1 -> 2 -> 3 -> 1 -> 2 -> 3 -> 4", 0.22)
            .with_color(GRAY_B),
        Vec3::new(0.0, -3.1, 0.0),
    );

    let mut task_checkmarks = Vec::new();
    for idx in 0..3 {
        let check_id = scene.add_tattva(
            Label::new("✓", 0.34).with_color(GREEN_B),
            Vec3::new(5.55, 1.0 - idx as f32 * 0.72, 0.0),
        );
        scene.hide(check_id);
        task_checkmarks.push(check_id);
    }

    let mut timeline = Timeline::new();

    let reveal_duration = 3.2;
    timeline
        .animate(chart_id)
        .at(0.4)
        .for_duration(reveal_duration)
        .ease(Ease::Linear)
        .reveal_to(1.0)
        .spawn();

    timeline
        .animate(chart_id)
        .at(chart_start + reveal_duration)
        .for_duration(chart_duration)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    if let Some(reason_visit) = arrivals.iter().find(|arrival| arrival.node_index == 2) {
        timeline.call_at(reason_visit.time, |_| {
            // Placeholder hook for future "node finished processing" scenes.
        });
    }

    if let Some(done_time) = completion_time {
        for (idx, check_id) in task_checkmarks.into_iter().enumerate() {
            timeline.call_at(done_time + idx as f32 * 0.45, move |scene| {
                scene.show(check_id);
            });
        }
    }

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
