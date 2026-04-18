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

    // Title
    scene.add_tattva(
        murali::frontend::collection::text::label::Label::new("Typst Formula Morph", 0.34)
            .with_color(WHITE),
        3.0 * UP,
    );

    // Subtitle
    scene.add_tattva(
        murali::frontend::collection::text::label::Label::new(
            "Vector formula morph with automatic staging",
            0.18,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, 2.45, 0.0),
    );

    // Content
    let source =
        VectorTypstEquation::new("$(a + b)^2$", 1.2).with_color(BLUE_B);
    let target = VectorTypstEquation::new("$a^2 + 2 a b + b^2$", 1.0)
        .with_color(GOLD_C);

    let source_handle = scene.add_vector_formula_typst(source);
    let target_handle = scene.add_vector_formula_typst(target);

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // Timeline
    let mut timeline = Timeline::new();
    timeline.morph_vector_formulas(
        &source_handle,
        &target_handle,
        &mut scene,
        1.0,
        3.0,
        Ease::InOutCubic,
    );

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}
