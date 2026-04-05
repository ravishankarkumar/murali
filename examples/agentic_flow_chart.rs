use glam::{Vec3, Vec4, vec2};
use murali::App;
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
        Label::new("Agentic Loop / Flow Chart", 0.34).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
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
            .with_stroke_color(Vec4::new(0.98, 0.72, 0.34, 1.0))
            .with_placement(FlowNodePlacement::BelowPrevious),
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
        .with_active_edge_color(Vec4::new(0.98, 0.80, 0.30, 1.0))
        .with_pulse_color(Vec4::new(1.0, 0.96, 0.84, 1.0));

    let chart_start = 0.2;
    let chart_duration = 5.6;
    let arrivals = chart.node_arrivals(chart_start, chart_duration);
    let completion_time = chart.completion_time(chart_start, chart_duration);

    let chart_id = scene.add_tattva(chart, Vec3::new(0.0, 0.25, 0.0));

    scene.add_tattva(
        Label::new("Path: 0 -> 1 -> 2 -> 3 -> 1 -> 2 -> 3 -> 4", 0.22)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.1, 0.0),
    );

    let mut task_checkmarks = Vec::new();
    for idx in 0..3 {
        let check_id = scene.add_tattva(
            Label::new("✓", 0.34).with_color(Vec4::new(0.72, 0.95, 0.66, 1.0)),
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

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
