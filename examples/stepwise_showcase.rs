use std::sync::Arc;

use glam::Vec3;

use murali::colors::{GRAY_B, TEAL_C, WHITE};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::indicate::Indicate;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::neural_network_diagram::{
    IndicationStyle, NeuralNetworkDiagram,
};
use murali::frontend::collection::storytelling::stepwise::{
    model::{Direction, StepContent},
    state::StepState,
    Stepwise, StepwiseLayout,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::TattvaId;
use murali::positions::CAMERA_DEFAULT_POS;
use murali::projection::{Project, ProjectionCtx};

const TITLE_POS: Vec3 = Vec3::new(0.0, 3.0, 0.0);
const SUBTITLE_POS: Vec3 = Vec3::new(0.0, 2.45, 0.0);
const FLOW_POS: Vec3 = Vec3::new(-5.0, 0.0, 0.0);

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let timeline_simple = showcase_linear_flow(&mut scene, 0.0);
    let timeline_feedback = showcase_feedback_flow(&mut scene, 10.5);
    let timeline_custom = showcase_custom_content_flow(&mut scene, 22.0);

    scene.set_timeline("showcase_linear", timeline_simple);
    scene.set_timeline("showcase_feedback", timeline_feedback);
    scene.set_timeline("showcase_custom", timeline_custom);

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene.camera_mut().set_view_width(16.0);

    App::new()?.with_scene(scene).run_app()
}

fn showcase_linear_flow(scene: &mut Scene, start: f32) -> Timeline {
    let title_id = scene.add_tattva(
        Label::new("Stepwise: Linear Reveal", 0.34).with_color(WHITE),
        TITLE_POS,
    );
    let subtitle_id = scene.add_tattva(
        Label::new("A clean progression from observation to reflection.", 0.18)
            .with_color(GRAY_B),
        SUBTITLE_POS,
    );

    let stepwise_id = scene.add_tattva(
        Stepwise::from_script(|s| {
            let observe = s.step("Observe");
            let reason = s.step("Reason");
            let plan = s.step("Plan");
            let act = s.step("Act");
            let reflect = s.step("Reflect");

            s.connect(observe, reason);
            s.connect(reason, plan);
            s.connect(plan, act);
            s.connect(act, reflect);
        })
        .with_layout(StepwiseLayout::horizontal(1.0)),
        FLOW_POS,
    );

    let mut timeline = Timeline::new();
    animate_text_block(&mut timeline, title_id, subtitle_id, start, 1.4, 7.0);
    animate_stepwise_block(&mut timeline, stepwise_id, start + 1.2, 2.8, start + 4.5, 3.0);
    timeline
}

fn showcase_feedback_flow(scene: &mut Scene, start: f32) -> Timeline {
    let title_id = scene.add_tattva(
        Label::new("Stepwise: Routed Feedback", 0.34).with_color(WHITE),
        TITLE_POS,
    );
    let subtitle_id = scene.add_tattva(
        Label::new("Back-path routing makes loops readable without breaking the story.", 0.18)
            .with_color(GRAY_B),
        SUBTITLE_POS,
    );

    let stepwise_id = scene.add_tattva(
        Stepwise::from_script(|s| {
            let input = s.step("Input Layer");
            let process = s.step("Processing");
            let feedback = s.step("Feedback Check");
            let refine = s.step("Processing 2");
            let publish = s.step("Feedback 3");

            s.connect(input, process);
            s.connect(process, feedback);
            s.connect(feedback, refine);
            s.connect(refine, publish);
            s.connect(refine, process)
                .route(vec![Direction::Down, Direction::Left]);

            s.with_sequence(vec![
                input, process, feedback, refine, process, feedback, refine, publish,
            ]);
        })
        .with_layout(StepwiseLayout::horizontal(1.2)),
        Vec3::new(-8.0, 0.0, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate_camera()
        .at(start + 0.2)
        .for_duration(1.2)
        .ease(Ease::InOutQuad)
        .zoom_to(2.0 / 3.0)
        .spawn();

    animate_text_block(&mut timeline, title_id, subtitle_id, start, 1.4, 8.2);
    animate_stepwise_block(&mut timeline, stepwise_id, start + 1.2, 3.0, start + 4.9, 3.8);

    timeline
        .animate_camera()
        .at(start + 9.0)
        .for_duration(1.1)
        .ease(Ease::InOutQuad)
        .zoom_to(3.0/2.0)
        .spawn();

    timeline
}

fn showcase_custom_content_flow(scene: &mut Scene, start: f32) -> Timeline {
    let title_id = scene.add_tattva(
        Label::new("Stepwise: Embedded Content", 0.34).with_color(WHITE),
        TITLE_POS,
    );
    let subtitle_id = scene.add_tattva(
        Label::new("A node can host rich projected content and still join the same journey.", 0.18)
            .with_color(GRAY_B),
        SUBTITLE_POS,
    );

    let stepwise_id = scene.add_tattva(
        Stepwise::from_script(|s| {
            let input = s.step("Data Input");
            let neural_content: Arc<dyn StepContent> = Arc::new(NeuralStepContent::new());
            let neural = s.step_with_content("Neural Inference", neural_content);
            let output = s.step("Result Output");

            s.connect(input, neural);
            s.connect(neural, output);
        })
        .with_layout(StepwiseLayout::horizontal(0.8))
        .with_style({
            let mut style = murali::frontend::collection::storytelling::stepwise::StepwiseStyle::default();
            style.signal_color = TEAL_C;
            style
        }),
        FLOW_POS,
    );

    let mut timeline = Timeline::new();
    animate_text_block(&mut timeline, title_id, subtitle_id, start, 1.4, 8.0);
    animate_stepwise_block(&mut timeline, stepwise_id, start + 1.2, 2.8, start + 4.4, 3.4);
    timeline
}

fn animate_text_block(
    timeline: &mut Timeline,
    title_id: TattvaId,
    subtitle_id: TattvaId,
    start: f32,
    write_duration: f32,
    hold_until: f32,
) {
    timeline
        .animate(title_id)
        .at(start)
        .for_duration(write_duration)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(subtitle_id)
        .at(start + 0.35)
        .for_duration(write_duration)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(title_id)
        .at(start + hold_until)
        .for_duration(1.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    timeline
        .animate(subtitle_id)
        .at(start + hold_until + 0.15)
        .for_duration(1.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();
}

fn animate_stepwise_block(
    timeline: &mut Timeline,
    stepwise_id: TattvaId,
    reveal_at: f32,
    reveal_duration: f32,
    signal_at: f32,
    signal_duration: f32,
) {
    timeline
        .animate(stepwise_id)
        .at(reveal_at)
        .for_duration(reveal_duration)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    timeline
        .animate(stepwise_id)
        .at(signal_at)
        .for_duration(signal_duration)
        .ease(Ease::Linear)
        .signal_to(1.0)
        .spawn();

    timeline
        .animate(stepwise_id)
        .at(signal_at + signal_duration + 0.8)
        .for_duration(1.2)
        .ease(Ease::InOutQuad)
        .propagate_to(0.0)
        .spawn();
}

#[derive(Debug)]
struct NeuralStepContent {
    diagram: NeuralNetworkDiagram,
}

impl NeuralStepContent {
    fn new() -> Self {
        let diagram = NeuralNetworkDiagram::new(vec![3, 4, 2])
            .with_layer_spacing(0.55)
            .with_node_spacing(0.32)
            .with_node_radius(0.08)
            .with_indication_style(IndicationStyle::Loop(2));
        Self { diagram }
    }
}

impl StepContent for NeuralStepContent {
    fn project(&self, ctx: &mut ProjectionCtx, state: &StepState) {
        let opacity = match state {
            StepState::Pending => 0.0,
            StepState::Active { t } => *t,
            StepState::Completed => 0.8,
        };
        ctx.with_opacity(opacity, |ctx| {
            self.diagram.project(ctx);
        });
    }

    fn project_indicated(&self, ctx: &mut ProjectionCtx, state: &StepState, intensity: f32) {
        let opacity = match state {
            StepState::Pending => 0.0,
            StepState::Active { t } => *t,
            StepState::Completed => 0.8,
        };
        ctx.with_opacity(opacity, |ctx| {
            self.diagram.project_indicated(ctx, intensity);
        });
    }
}
