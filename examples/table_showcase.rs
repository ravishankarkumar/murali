/// Table showcase demonstrating Manim-style table implementation
use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::table::Table;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Table Showcase", 0.4)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Left table - Person data
    let left_table = Table::new(vec![
        vec!["Alice", "28", "NYC"],
        vec!["Bob", "34", "LA"],
        vec!["Charlie", "25", "Chicago"],
    ])
    .with_col_labels(vec!["Name", "Age", "City"])
    .with_row_labels(vec!["Person 1", "Person 2", "Person 3"])
    .with_line_color(Vec4::new(0.44, 0.84, 0.71, 1.0))
    .with_text_color(Vec4::new(0.96, 0.98, 0.99, 1.0))
    .with_text_height(0.25)
    .with_h_buff(0.3)
    .with_v_buff(0.2)
    .with_outer_lines(true);

    let left_table_id = scene.add_tattva(left_table, Vec3::new(-3.5, 0.8, 0.0));

    scene.add_tattva(
        Label::new("Person Data", 0.22)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(-3.5, -0.8, 0.0),
    );

    // Right table - Quarterly data
    let right_table = Table::new(vec![
        vec!["100", "150", "120", "180"],
        vec!["200", "180", "220", "210"],
    ])
    .with_col_labels(vec!["Q1", "Q2", "Q3", "Q4"])
    .with_row_labels(vec!["Product A", "Product B"])
    .with_line_color(Vec4::new(0.78, 0.82, 0.90, 1.0))
    .with_text_color(Vec4::new(0.96, 0.98, 0.99, 1.0))
    .with_text_height(0.22)
    .with_h_buff(0.25)
    .with_v_buff(0.15)
    .with_outer_lines(true)
    .with_labels_inside(true);

    let right_table_id = scene.add_tattva(right_table, Vec3::new(3.5, 0.8, 0.0));

    scene.add_tattva(
        Label::new("Quarterly Sales", 0.22)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(3.5, -0.8, 0.0),
    );

    let mut timeline = Timeline::new();

    // Animate left table
    timeline
        .animate(left_table_id)
        .at(0.5)
        .for_duration(1.0)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(-3.5, 0.8, 0.0))
        .spawn();

    // Animate right table
    timeline
        .animate(right_table_id)
        .at(1.2)
        .for_duration(1.0)
        .ease(Ease::InOutQuad)
        .move_to(Vec3::new(3.5, 0.8, 0.0))
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
