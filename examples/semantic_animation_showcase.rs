use glam::{Vec3, Vec4};
use murali::App;
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
            .with_color(Vec4::new(0.95, 0.97, 0.99, 1.0)),
        Vec3::new(0.0, 3.2, 0.0),
    );

    let square_id = scene.add_tattva(
        Square::new(1.0, Vec4::new(0.96, 0.42, 0.28, 1.0)),
        Vec3::new(-5.0, 1.4, 0.0),
    );

    let circle_id = scene.add_tattva(
        Circle::new(0.62, 48, Vec4::new(0.26, 0.70, 0.44, 1.0)),
        Vec3::new(-1.8, 1.4, 0.0),
    );

    let eq_from_id = scene.add_tattva(
        EquationLayout::new(
            vec![
                EquationPart::new("y").with_key("lhs"),
                EquationPart::new("=")
                    .with_key("eq")
                    .with_color(Vec4::new(0.94, 0.88, 0.46, 1.0)),
                EquationPart::new("Wx")
                    .with_key("expr")
                    .with_color(Vec4::new(0.37, 0.80, 0.97, 1.0)),
                EquationPart::new("+").with_key("plus"),
                EquationPart::new("b")
                    .with_key("bias")
                    .with_color(Vec4::new(0.98, 0.58, 0.34, 1.0)),
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
                    .with_color(Vec4::new(0.71, 0.56, 0.98, 1.0)),
                EquationPart::new("("),
                EquationPart::new("Wx")
                    .with_key("expr")
                    .with_color(Vec4::new(0.37, 0.80, 0.97, 1.0)),
                EquationPart::new("+").with_key("plus"),
                EquationPart::new("b")
                    .with_key("bias")
                    .with_color(Vec4::new(0.98, 0.58, 0.34, 1.0)),
                EquationPart::new(")").with_key("close"),
            ],
            0.34,
        ),
        Vec3::new(2.3, 1.6, 0.0),
    );
    if let Some(t) = scene.get_tattva_any_mut(eq_to_id) {
        let mut props = t.props().write();
        props.visible = false;
        props.opacity = 0.0;
    }

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
        .matrix_step_row(0, Vec4::new(0.22, 0.50, 0.96, 0.55), 0.35)
        .spawn();

    timeline
        .animate(matrix_id)
        .at(2.0)
        .for_duration(1.0)
        .ease(Ease::OutQuad)
        .matrix_step_cells(
            vec![(0, 1), (1, 2), (2, 0)],
            Vec4::new(0.98, 0.66, 0.22, 0.60),
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

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    App::new()?.with_scene(scene).run_app()
}
