use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::math::equation::{EquationLayout, EquationPart};
use murali::frontend::collection::math::matrix::Matrix;
use murali::frontend::collection::primitives::{circle::Circle, square::Square};
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("Milestone 6: semantic animation", 0.28)
            .with_color(WHITE),
        Vec3::new(0.0, 3.2, 0.0),
    );

    let square_id = scene.add_tattva(
        Square::new(1.0, ORANGE_D),
        Vec3::new(-5.0, 1.4, 0.0),
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.62, 48, GREEN_C),
        Vec3::new(-1.8, 1.4, 0.0),
    );

    let eq_from_id = scene.add_tattva(
        EquationLayout::new(
            vec![
                EquationPart::new("y").with_key("lhs"),
                EquationPart::new("=")
                    .with_key("eq")
                    .with_color(GOLD_B),
                EquationPart::new("Wx")
                    .with_key("expr")
                    .with_color(BLUE_B),
                EquationPart::new("+").with_key("plus"),
                EquationPart::new("b")
                    .with_key("bias")
                    .with_color(ORANGE_D),
            ],
            0.34,
        ),
        Vec3::new(2.3, 1.6, 0.0),
    );

    let eq_to_id = scene.add_tattva(
        EquationLayout::new(
            vec![
                EquationPart::new("softmax")
                    .with_key("fn")
                    .with_color(PURPLE_B),
                EquationPart::new("("),
                EquationPart::new("Wx")
                    .with_key("expr")
                    .with_color(BLUE_B),
                EquationPart::new("+").with_key("plus"),
                EquationPart::new("b")
                    .with_key("bias")
                    .with_color(ORANGE_D),
                EquationPart::new(")").with_key("close"),
            ],
            0.34,
        ),
        Vec3::new(2.3, 1.6, 0.0),
    );
    scene.hide_tattva(eq_to_id);

    let matrix_id = scene.add_tattva(
        Matrix::new(
            vec![
                vec!["0.2", "0.7", "0.1"],
                vec!["0.1", "0.2", "0.7"],
                vec!["0.8", "0.1", "0.1"],
            ],
            0.34,
        ),
        Vec3::new(2.2, -1.25, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(circle_id)
        .at(0.0)
        .for_duration(1.6)
        .ease(Ease::InOutQuad)
        .match_transform(square_id)
        .spawn();

    timeline
        .animate(circle_id)
        .at(1.7)
        .for_duration(1.5)
        .ease(Ease::InOutQuad)
        .morph_from(square_id)
        .spawn();

    timeline
        .animate(eq_to_id)
        .at(0.5)
        .for_duration(2.2)
        .ease(Ease::InOutQuad)
        .equation_continuity_from(eq_from_id)
        .spawn();

    timeline
        .animate(matrix_id)
        .at(0.8)
        .for_duration(1.0)
        .ease(Ease::OutQuad)
        .matrix_step_row(0, BLUE_D, 0.35)
        .spawn();

    timeline
        .animate(matrix_id)
        .at(2.0)
        .for_duration(1.0)
        .ease(Ease::OutQuad)
        .matrix_step_cells(
            vec![(0, 1), (1, 2), (2, 0)],
            YELLOW_B,
            0.28,
        )
        .spawn();

    timeline
        .animate(title_id)
        .at(2.8)
        .for_duration(1.2)
        .ease(Ease::InOutQuad)
        .fade_to(0.45)
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
