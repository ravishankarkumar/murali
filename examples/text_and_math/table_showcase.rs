/// Table showcase demonstrating collection-level table build animations
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::table::Table;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("Table Showcase - Animated", 0.4).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 1.0);

    let left_table = Table::new(vec![
        vec!["Alice", "28", "NYC"],
        vec!["Bob", "34", "LA"],
        vec!["Charlie", "25", "Chicago"],
    ])
    .with_col_labels(vec!["Name", "Age", "City"])
    .with_row_labels(vec!["Person 1", "Person 2", "Person 3"])
    .with_line_color(TEAL_C)
    .with_text_color(WHITE)
    .with_text_height(0.25)
    .with_h_buff(0.3)
    .with_v_buff(0.2)
    .with_outer_lines(true)
    .with_title("Person Data")
    .with_write_progress(0.0);
    let left_table_id = scene.add_tattva(left_table, Vec3::new(-3.5, 0.8, 0.0));

    let right_table = Table::new(vec![
        vec!["100", "150", "120", "180"],
        vec!["200", "180", "220", "210"],
    ])
    .with_col_labels(vec!["Q1", "Q2", "Q3", "Q4"])
    .with_row_labels(vec!["Product A", "Product B"])
    .with_line_color(GRAY_A)
    .with_text_color(WHITE)
    .with_text_height(0.22)
    .with_h_buff(0.25)
    .with_v_buff(0.15)
    .with_outer_lines(true)
    .with_labels_inside(false)
    .with_title("Quarterly Sales")
    .with_write_progress(0.0);
    let right_table_id = scene.add_tattva(right_table, Vec3::new(3.5, 0.8, 0.0));

    let mut timeline = Timeline::new();
    timeline
        .animate(left_table_id)
        .at(0.5)
        .for_duration(3.0)
        .ease(Ease::InOutQuad)
        .write_table()
        .spawn();

    timeline
        .animate(right_table_id)
        .at(4.0)
        .for_duration(3.0)
        .ease(Ease::InOutQuad)
        .write_table()
        .spawn();

    timeline
        .animate(left_table_id)
        .at(8.0)
        .for_duration(2.0)
        .ease(Ease::InOutQuad)
        .unwrite_table()
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
