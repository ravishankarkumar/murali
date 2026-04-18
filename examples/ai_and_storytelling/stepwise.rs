/// Example: Stepwise — agentic loop with a back-edge
///
/// Shows a 5-step agentic loop where Act can loop back to Reason.
/// The main sequence is linear (Observe→Reason→Plan→Act→Reflect).
/// The Act→Reason back-edge is declared as an extra connection so it
/// renders as a completed edge once Act is reached, without disrupting
/// the sequential build animation.
use glam::Vec3;
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::storytelling::stepwise::{
    Stepwise, layout::StepwiseLayout, script::stepwise,
};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Build a linear chain — no explicit connections so the engine
    // auto-generates sequence [0,1,2,3,4] and transitions 0→1→2→3→4.
    // The Act→Reason back-edge is added manually to the model after build
    // so it renders without affecting the topological sequence.
    let mut model = stepwise(|s| {
        s.step("Observe");
        s.step("Reason");
        s.step("Plan");
        s.step("Act");
        s.step("Reflect");
    });

    // Add the back-edge Act(3)→Reason(1) as an extra transition.
    // It won't be part of the sequence so it won't affect timing —
    // it just appears as Completed once Act is done.
    use murali::frontend::collection::storytelling::stepwise::model::{Direction, Transition};
    model.transitions.push(Transition {
        from: 3,
        to: 1,
        route: Some(vec![
            Direction::Up,
            Direction::Left, /*, Direction::Left, Direction::Down*/
        ]),
    });

    let sw = Stepwise::new(model)
        .with_layout(StepwiseLayout::default())
        .with_debug(false);

    let sw_id = scene.add_tattva(sw, 4.0 * LEFT);

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 12.0);

    let mut timeline = Timeline::new();

    // Phase 1: build the diagram
    timeline
        .animate(sw_id)
        .at(0.5)
        .for_duration(6.0)
        .ease(Ease::Linear)
        .propagate_to(1.0)
        .spawn();

    // Phase 2: signal flow
    timeline
        .animate(sw_id)
        .at(8.0)
        .for_duration(4.0)
        .ease(Ease::InOutQuad)
        .signal_to(1.0)
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
