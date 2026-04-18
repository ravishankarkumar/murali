use glam::{Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::{axes::Axes, number_plane::NumberPlane};
use murali::frontend::collection::graph::{
    function_graph::FunctionGraph, scatter_plot::ScatterPlot,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn cubic(x: f32) -> f32 {
    0.08 * x * x * x - 0.55 * x
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    let title_id = scene.add_tattva(
        Label::new("Graph On 2D Axes", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.add_tattva(
        NumberPlane::new((-5.0, 5.0), (-3.5, 3.5)).with_step(1.0),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Axes::new((-5.0, 5.0), (-3.5, 3.5))
            .with_step(1.0)
            .with_thickness(0.03)
            .with_tick_size(0.16)
            .with_color(GRAY_A),
        Vec3::ZERO,
    );

    scene.add_tattva(
        FunctionGraph::new((-4.6, 4.6), cubic).with_samples(220),
        Vec3::ZERO,
    );

    scene.add_tattva(
        ScatterPlot::new(vec![
            vec2(-3.8, cubic(-3.8)),
            vec2(-2.4, cubic(-2.4)),
            vec2(-1.2, cubic(-1.2)),
            vec2(0.0, cubic(0.0)),
            vec2(1.4, cubic(1.4)),
            vec2(2.8, cubic(2.8)),
            vec2(4.0, cubic(4.0)),
        ]),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Label::new("y = 0.08x^3 - 0.55x", 0.24).with_color(BLUE_B),
        Vec3::new(2.8, 2.9, 0.0),
    );

    scene.add_tattva(
        Label::new("Sampled points overlaid on the same axes.", 0.20)
            .with_color(GRAY_B),
        Vec3::new(0.0, -3.9, 0.0),
    );

    scene.add_tattva(
        Label::new("x", 0.28).with_color(ORANGE_B),
        Vec3::new(5.25, -0.18, 0.0),
    );

    scene.add_tattva(
        Label::new("y", 0.28).with_color(BLUE_B),
        Vec3::new(0.18, 3.75, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.5);

    App::new()?.with_scene(scene).run_app()
}
