/// Simple table test without animation
use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::table::Table;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Table Test - No Animation", 0.4)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Simple table - fully visible
    let table = Table::new(vec![
        vec!["Alice", "28", "NYC"],
        vec!["Bob", "34", "LA"],
    ])
    .with_col_labels(vec!["Name", "Age", "City"])
    .with_row_labels(vec!["Person 1", "Person 2"])
    .with_line_color(Vec4::new(0.44, 0.84, 0.71, 1.0))
    .with_text_color(Vec4::new(0.96, 0.98, 0.99, 1.0))
    .with_text_height(0.25)
    .with_h_buff(0.3)
    .with_v_buff(0.2)
    .with_outer_lines(true)
    .with_write_progress(1.0);  // Fully visible

    scene.add_tattva(table, Vec3::new(0.0, 0.0, 0.0));

    App::new()?.with_scene(scene).run_app()
}
