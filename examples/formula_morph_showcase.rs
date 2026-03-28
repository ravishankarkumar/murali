use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::math::equation::VectorEquation;
use murali::App;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // 1. Create the two equations (mathematically equivalent but different layout)
    // We use Typst math syntax: $...$
    let eq1 = VectorEquation::new("$(a + b)^2$", 1.2)
        .with_color(Vec4::new(0.4, 0.7, 1.0, 1.0)); // Blueish
    
    let eq2 = VectorEquation::new("$a^2 + 2 a b + b^2$", 1.0)
        .with_color(Vec4::new(1.0, 0.8, 0.4, 1.0)); // Orangeish

    // 2. Spawn them into the scene
    // Note: They are spawned as individual character Tattvas.
    let sources = eq1.spawn(&mut scene);
    let targets = eq2.spawn(&mut scene);

    // Hide target equation initially
    for &id in &targets {
        if let Some(t) = scene.get_tattva_any_mut(id) {
            let mut props = t.props().write();
            props.opacity = 0.0;
            props.visible = false;
        }
    }

    let mut timeline = Timeline::new();

    // 3. Perform the Smart Morph
    // This will match 'a' to 'a', 'b' to 'b', '+' to '+', etc.
    // It also handles moving them to their new positions in the expanded formula.
    timeline.morph_matching(
        sources,
        targets,
        &scene,
        1.5,           // Start at 1.5s
        3.0,           // Duration 3s
        Ease::InOutCubic,
    );

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    println!("Running Formula Morph Showcase...");
    println!("Formula: (a + b)^2  --->  a^2 + 2ab + b^2");
    println!("Command: cargo run --example formula_morph_showcase --features typst_embedded");

    App::new()?.with_scene(scene).run_app()
}
