/// Example: Stepwise Script API
///
/// Demonstrates the `stepwise(|s| { ... })` closure builder used as a
/// first-class Murali tattva — no manual updater, no Arc<Mutex<>>.
use glam::Vec3;
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::storytelling::stepwise::{
    Stepwise, layout::StepwiseLayout, model::Direction, script::stepwise,
};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let model = stepwise(|s| {
        let observe = s.step("Observe");
        let reason = s.step("Reason");
        let plan = s.step("Plan");
        let act = s.step("Act");
        let reflect = s.step("Reflect");

        s.connect(observe, reason);
        s.connect(reason, plan);
        s.connect(plan, act)
            .route(vec![Direction::Up, Direction::Right]);
        s.connect(act, reflect);
    });

    let sw = Stepwise::new(model)
        .with_layout(StepwiseLayout::vertical(0.4))
        .with_debug(true);

    let sw_id = scene.add_tattva(sw, Vec3::new(0.0, 3.5, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 12.0);

    let mut timeline = Timeline::new();

    // Phase 1: build the diagram (nodes write in, edges grow)
    timeline
        .animate(sw_id)
        .at(0.5)
        .for_duration(6.0)
        .ease(Ease::Linear)
        .propagate_to(1.0)
        .spawn();

    // Phase 2: signal flow — explain the concept on the fully-built diagram
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
