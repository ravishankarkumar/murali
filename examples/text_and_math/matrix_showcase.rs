use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::math::matrix::Matrix;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Matrix Showcase", 0.34).with_color(WHITE),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    let mut matrix_a = Matrix::new(
        vec![
            vec!["1", "2", "3"],
            vec!["0", "1", "4"],
            vec!["5", "6", "0"],
        ],
        0.42,
    );
    matrix_a.color = GOLD_B;
    matrix_a.bracket_color = BLUE_A;
    scene.add_tattva(matrix_a, 3.3 * LEFT + 0.65 * UP);

    let mut matrix_b = Matrix::new(
        vec![
            vec!["0.2", "0.7", "0.1"],
            vec!["0.1", "0.2", "0.7"],
            vec!["0.8", "0.1", "0.1"],
        ],
        0.34,
    );
    matrix_b.color = BLUE_B;
    matrix_b.bracket_color = WHITE;
    if let Some(cell) = matrix_b.cell_mut(0, 1) {
        cell.highlight = Some(YELLOW_B);
    }
    if let Some(cell) = matrix_b.cell_mut(1, 2) {
        cell.highlight = Some(YELLOW_B);
    }
    if let Some(cell) = matrix_b.cell_mut(2, 0) {
        cell.highlight = Some(YELLOW_B);
    }
    scene.add_tattva(matrix_b, 3.0 * RIGHT + 0.95 * UP);

    let mut vector = Matrix::new(vec![vec!["x"], vec!["y"], vec!["z"]], 0.40);
    vector.color = WHITE;
    vector.bracket_color = GRAY_A;
    scene.add_tattva(vector, 1.35 * DOWN);

    scene.add_tattva(
        Label::new(
            "Use this scene to validate native matrix brackets, spacing, and cell highlighting.",
            0.22,
        )
        .with_color(GRAY_A),
        3.0 * DOWN,
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    App::new()?.with_scene(scene).run_app()
}
