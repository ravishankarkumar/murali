use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::math::equation::{VectorLatexEquation, VectorTypstEquation};

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let source =
        VectorTypstEquation::new("$(a + b)^2$", 1.5).with_color(BLUE_B);
    let target = VectorLatexEquation::new(r"a^2 + 2ab + b^2", 1.45)
        .with_color(GOLD_C);

    let source_handle = scene.add_vector_typst(source);
    let target_handle = scene.add_vector_latex(target);

    let mut timeline = Timeline::new();
    timeline.morph_vector_equations(
        &source_handle,
        &target_handle,
        &mut scene,
        1.0,
        3.0,
        Ease::InOutCubic,
    );

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
