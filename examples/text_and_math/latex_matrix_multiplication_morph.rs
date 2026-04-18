use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::math::equation::VectorLatexEquation;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("LaTeX Matrix Multiplication Morph", 0.30)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    let source = VectorLatexEquation::new(
        r"\begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix} \times \begin{bmatrix} 2 & 0 \\ 1 & 2 \end{bmatrix}",
        1.10,
    )
    .with_color(BLUE_B);

    let target = VectorLatexEquation::new(r"\begin{bmatrix} 4 & 4 \\ 10 & 8 \end{bmatrix}", 1.10)
        .with_color(GOLD_C);

    let source_handle = scene.add_vector_latex(source);
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
