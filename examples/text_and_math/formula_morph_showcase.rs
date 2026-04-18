use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::math::equation::VectorTypstEquation;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // 1. Create the two equations (mathematically equivalent but different layout)
    // We use Typst math syntax: $...$
    let eq1 =
        VectorTypstEquation::new("$(a + b)^2$", 1.2).with_color(BLUE_B); // Blueish

    let eq2 = VectorTypstEquation::new("$a^2 + 2 a b + b^2$", 1.0)
        .with_color(GOLD_C); // Orangeish

    let source_handle = scene.add_vector_typst(eq1);
    let target_handle = scene.add_vector_typst(eq2);

    let mut timeline = Timeline::new();

    // 3. Perform the Smart Morph
    // This will match 'a' to 'a', 'b' to 'b', '+' to '+', etc.
    // It also handles moving them to their new positions in the expanded formula.
    timeline.morph_vector_equations(
        &source_handle,
        &target_handle,
        &mut scene,
        1.5, // Start at 1.5s
        3.0, // Duration 3s
        Ease::InOutCubic,
    );

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
