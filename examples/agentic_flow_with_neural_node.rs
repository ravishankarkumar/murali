use glam::{Vec2, Vec3, Vec4, vec2};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::agentic_flow_chart::{
    AgenticFlowChart, FlowChartDirection, FlowEdge, FlowNode, FlowNodeShape,
};
use murali::frontend::collection::ai::neural_network_diagram::NeuralNetworkDiagram;
use murali::frontend::collection::ai::signal_flow::SignalFlow;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::{Bounded, Bounds, Direction};
use murali::frontend::DirtyFlags;
use murali::projection::{Project, ProjectionCtx, RenderPrimitive};
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Debug)]
struct NeuralNodeContent {
    title: String,
    diagram: NeuralNetworkDiagram,
    flow_paths: Vec<Vec<Vec3>>,
    progress: Arc<Mutex<f32>>,
}

impl NeuralNodeContent {
    fn new(title: impl Into<String>, progress: Arc<Mutex<f32>>) -> Self {
        let mut diagram = NeuralNetworkDiagram::new(vec![3, 4, 2]);
        diagram.layer_spacing = 0.64;
        diagram.node_spacing = 0.30;
        diagram.node_radius = 0.055;
        diagram.edge_thickness = 0.010;
        diagram.node_color = Vec4::new(0.36, 0.84, 0.98, 1.0);
        diagram.edge_color = Vec4::new(0.46, 0.58, 0.72, 0.95);
        let flow_paths = diagram.all_path_points();

        Self {
            title: title.into(),
            diagram,
            flow_paths,
            progress,
        }
    }
}

impl Project for NeuralNodeContent {
    fn project(&self, ctx: &mut ProjectionCtx) {
        ctx.emit(RenderPrimitive::Text {
            content: self.title.clone(),
            height: 0.18,
            color: Vec4::new(0.96, 0.98, 0.99, 1.0),
            offset: Vec3::new(0.0, 0.64, 0.0),
        });

        self.diagram.project(ctx);

        let progress = *self.progress.lock();
        SignalFlow::from_paths(self.flow_paths.clone())
            .with_progress(progress)
            .with_edge_color(Vec4::new(0.98, 0.76, 0.30, 0.95))
            .with_pulse_color(Vec4::new(1.0, 0.96, 0.82, 1.0))
            .project(ctx);
    }
}

impl Bounded for NeuralNodeContent {
    fn local_bounds(&self) -> Bounds {
        self.diagram.local_bounds().union(&Bounds::from_center_size(
            Vec2::new(0.0, 0.64),
            vec2(1.9, 0.28),
        ))
    }
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("Agentic Flow With Neural Node", 0.34)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    let neural_progress = Arc::new(Mutex::new(0.0));
    let neural_content = NeuralNodeContent::new("Neural Network", neural_progress.clone());

    let chart_position = Vec3::new(-0.35, 0.35, 0.0);
    let nodes = vec![
        FlowNode::new("Start")
            .with_shape(FlowNodeShape::Pill)
            .with_fill_color(Vec4::new(0.16, 0.36, 0.31, 1.0))
            .with_stroke_color(Vec4::new(0.44, 0.84, 0.71, 1.0)),
        FlowNode::new("Retrieve")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(Vec4::new(0.19, 0.22, 0.35, 1.0))
            .with_stroke_color(Vec4::new(0.44, 0.62, 0.95, 1.0)),
        FlowNode::new("")
            .with_shape(FlowNodeShape::Rounded)
            .with_size(vec2(3.35, 2.0))
            .with_fill_color(Vec4::new(0.17, 0.20, 0.28, 1.0))
            .with_stroke_color(Vec4::new(0.52, 0.74, 0.98, 1.0))
            .with_content(neural_content)
            .with_content_padding(vec2(0.24, 0.18)),
        FlowNode::new("Answer")
            .with_shape(FlowNodeShape::Rounded)
            .with_fill_color(Vec4::new(0.20, 0.24, 0.30, 1.0))
            .with_stroke_color(Vec4::new(0.78, 0.82, 0.90, 1.0)),
        FlowNode::new("End")
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
            FlowEdge::new(3, 4),
        ])
        .with_flow_path(vec![0, 1, 2, 3, 4])
        .with_progress(0.0)
        .with_node_gap(0.95)
        .with_lane_gap(0.85)
        .with_active_edge_color(Vec4::new(0.98, 0.80, 0.30, 1.0))
        .with_pulse_color(Vec4::new(1.0, 0.96, 0.84, 1.0));

    let chart_id = scene.add_tattva(chart, chart_position);

    let checkmark_id = scene.add_tattva(
        Label::new("✓", 0.42).with_color(Vec4::new(0.72, 0.95, 0.66, 1.0)),
        Vec3::new(6.2, 0.78, 0.0),
    );
    scene.hide(checkmark_id);

    let task_done_id = scene.add_tattva(
        Label::new("Task completed", 0.26).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::new(6.2, 0.18, 0.0),
    );
    scene.hide(task_done_id);

    scene.add_tattva(
        Label::new(
            "The flow pauses at the embedded neural-network node until three inference loops complete.",
            0.20,
        )
        .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.2, 0.0),
    );

    let chart_start = 0.25;
    let chart_to_model_duration = 1.8;
    let model_progress = 0.5;
    let nn_loop_duration = 0.7;
    let nn_loops = 3;
    let nn_total_duration = nn_loop_duration * nn_loops as f32;
    let chart_resume_time = chart_start + chart_to_model_duration + nn_total_duration;
    let chart_to_end_duration = 1.8;
    let end_time = chart_resume_time + chart_to_end_duration;

    let mut timeline = Timeline::new();
    timeline
        .animate(chart_id)
        .at(chart_start)
        .for_duration(chart_to_model_duration)
        .ease(Ease::InOutQuad)
        .propagate_to(model_progress)
        .spawn();

    let progress_state = neural_progress.clone();
    let animated_chart_id = chart_id;
    timeline.call_during(
        chart_start + chart_to_model_duration,
        nn_total_duration,
        move |scene, t| {
            let scaled = t * nn_loops as f32;
            let local = if t >= 1.0 { 1.0 } else { scaled.fract() };
            *progress_state.lock() = local;
            if let Some(chart) = scene.get_tattva_typed_mut::<AgenticFlowChart>(animated_chart_id) {
                chart.mark_dirty(DirtyFlags::GEOMETRY | DirtyFlags::STYLE);
            }
        },
    );

    timeline
        .animate(chart_id)
        .at(chart_resume_time)
        .for_duration(chart_to_end_duration)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    timeline.call_at(end_time, move |scene| {
        scene.show(checkmark_id);
        scene.show(task_done_id);
    });

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.5);

    App::new()?.with_scene(scene).run_app()
}
