use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::text::latex::Latex;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("LaTeX Matrix Showcase", 0.34).with_color(WHITE),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.add_tattva(
        Latex::new(
            r"\mathbf{A} = \begin{bmatrix} 1 & 2 & 3 \\ 0 & 1 & 4 \\ 5 & 6 & 0 \end{bmatrix}",
            0.72,
        )
        .with_color(GOLD_B),
        Vec3::new(0.0, 1.65, 0.0),
    );

    scene.add_tattva(
        Latex::new(
            r"\det(\mathbf{A}) = \begin{vmatrix} a & b \\ c & d \end{vmatrix} = ad - bc",
            0.60,
        )
        .with_color(BLUE_B),
        Vec3::new(0.0, 0.35, 0.0),
    );

    scene.add_tattva(
        Latex::new(
            r"\begin{bmatrix} x' \\ y' \end{bmatrix} = \begin{bmatrix} \cos\theta & -\sin\theta \\ \sin\theta & \cos\theta \end{bmatrix} \begin{bmatrix} x \\ y \end{bmatrix}",
            0.64,
        )
        .with_color(WHITE),
        Vec3::new(0.0, -1.15, 0.0),
    );

    scene.add_tattva(
        Label::new(
            "Use this scene to validate matrix brackets, row spacing, and multi-block LaTeX layout.",
            0.22,
        )
        .with_color(GRAY_A),
        Vec3::new(0.0, -3.05, 0.0),
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    App::new()?.with_scene(scene).run_app()
}
