use glam::Vec3;
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::storytelling::stepwise::{
    Stepwise, StepwiseLayout, model::Direction, script::stepwise,
};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Define a 3-node model with a back-path from C -> A
    let model = stepwise(|s| {
        let a = s.step("Input Layer");
        let b = s.step("Processing");
        let c = s.step("Feedback Check");
        let d = s.step("Processing 2");
        let e = s.step("Feedback 3");

        // Forward path
        s.connect(a, b);
        s.connect(b, c);
        s.connect(c, d);
        s.connect(d, e);

        // Back path: D(3) -> A(0): 3 hops left.
        // Up exits top of D. Three Lefts step D->C->B->A by node center.
        // Spatial anchor enters A from the top.
        s.connect(d, b).route(vec![
            Direction::Down,
            Direction::Left,
            // Direction::Left,
            // Direction::Left,
        ]);

        // Explicit journey: A -> B -> C -> D -> C -> D -> E
        s.with_sequence(vec![a, b, c, d, b, c, d, e]);
    });

    let sw = Stepwise::new(model)
        .with_layout(StepwiseLayout::horizontal(2.0))
        .with_debug(true);

    let sw_id = scene.add_tattva(sw, 2.0 * LEFT);

    let mut timeline = Timeline::new();

    // 1. Reveal nodes & connections (0.0 to 1.0)
    timeline
        .animate(sw_id)
        .at(0.5)
        .for_duration(3.0)
        .ease(Ease::InOutQuad)
        .propagate_to(1.0)
        .spawn();

    // 2. Journey signal (A -> B -> C -> A)
    timeline
        .animate(sw_id)
        .at(4.0)
        .for_duration(6.0)
        .ease(Ease::Linear)
        .signal_to(1.0)
        .spawn();

    scene.set_timeline("main", timeline);

    scene.camera_mut().position = Vec3::new(0.0, -1.0, 10.0);

    let current_width = scene.camera_mut().view_width();
    scene.camera_mut().set_view_width(current_width * 2.5);

    App::new()?.with_scene(scene).run_app()
}
