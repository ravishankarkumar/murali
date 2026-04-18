use glam::Vec3;
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::animation::indicate::Indicate;
use murali::frontend::collection::ai::neural_network_diagram::{
    IndicationStyle, NeuralNetworkDiagram,
};
use murali::frontend::collection::storytelling::stepwise::{
    Stepwise, StepwiseLayout, model::StepContent, script::stepwise, state::StepState,
};
use murali::projection::{Project, ProjectionCtx};

/// A specialized StepContent that wraps a NeuralNetworkDiagram
/// and exposes its Indicate behavior to Stepwise.
#[derive(Debug)]
struct NeuralStepContent {
    diagram: NeuralNetworkDiagram,
}

impl NeuralStepContent {
    fn new() -> Self {
        let diagram = NeuralNetworkDiagram::new(vec![3, 4, 2])
            .with_layer_spacing(0.6)
            .with_node_spacing(0.35)
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
            // Forward the indication call to the diagram
            self.diagram.project_indicated(ctx, intensity);
        });
    }

    fn draws_own_background(&self) -> bool {
        false
    }
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Define the model using the DSL
    let model = stepwise(|s| {
        let input = s.step("Data Input");
        let nn = s.step_with_content("Neural Inference", Box::new(NeuralStepContent::new()));
        let output = s.step("Result Output");

        s.connect(input, nn);
        s.connect(nn, output);
    });

    let sw = Stepwise::new(model)
        .with_layout(StepwiseLayout::horizontal(0.5))
        .with_debug(true);

    let sw_id = scene.add_tattva(sw, 3.0 * LEFT);

    // Animation: Build the chart, then run the signal
    let mut timeline = Timeline::new();

    // 1. Reveal nodes (0.0 to 1.0)
    timeline
        .animate(sw_id)
        .at(0.5)
        .for_duration(3.0)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    // 2. Play the signal (0.0 to 1.0)
    // The signal will "hit" each node, triggering the Indicate transition.
    // For normal nodes, this is a pulse.
    // For the Neural Inference node, it triggers the internal firing loop.
    timeline
        .animate(sw_id)
        .at(3.0)
        .for_duration(4.0)
        .ease(Ease::Linear)
        .signal_to(1.0)
        .spawn();

    scene.set_timeline("main", timeline);

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
