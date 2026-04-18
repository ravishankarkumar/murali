use glam::{Vec2, Vec3, Vec4, vec2};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::composite::number_plane::NumberPlane;
use murali::frontend::collection::graph::{
    function_graph::FunctionGraph, parametric_curve::ParametricCurve, scatter_plot::ScatterPlot,
};
use murali::frontend::collection::math::{
    equation::{EquationLayout, EquationPart},
    matrix::Matrix,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn parabola(x: f32) -> f32 {
    0.18 * x * x - 1.2
}

fn spiral(t: f32) -> Vec2 {
    let r = 0.15 * t;
    vec2(r * t.cos(), r * t.sin())
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        NumberPlane::new((-5.0, 5.0), (-3.0, 3.0)).with_step(1.0),
        Vec3::new(-3.2, 0.2, 0.0),
    );

    scene.add_tattva(
        FunctionGraph::new((-4.5, 4.5), parabola).with_samples(160),
        Vec3::new(-3.2, 0.2, 0.0),
    );

    scene.add_tattva(
        ScatterPlot::new(vec![
            vec2(-3.5, 1.0),
            vec2(-2.0, -0.4),
            vec2(-0.8, -1.0),
            vec2(1.1, -0.7),
            vec2(2.6, 0.2),
            vec2(3.7, 1.5),
        ]),
        Vec3::new(-3.2, 0.2, 0.0),
    );

    scene.add_tattva(
        ParametricCurve::new((0.0, 10.0 * std::f32::consts::PI), spiral).with_samples(220),
        Vec3::new(2.2, 0.6, 0.0),
    );

    scene.add_tattva(
        Matrix::new(
            vec![
                vec!["1", "0", "1"],
                vec!["0", "1", "1"],
                vec!["2", "-1", "3"],
            ],
            0.32,
        ),
        Vec3::new(4.5, -1.6, 0.0),
    );

    scene.add_tattva(
        EquationLayout::new(
            vec![
                EquationPart::new("y"),
                EquationPart::new("=").with_color(GOLD_B),
                EquationPart::new("0.18x^2").with_color(BLUE_B),
                EquationPart::new("-"),
                EquationPart::new("1.2").with_color(ORANGE_D),
            ],
            0.30,
        ),
        Vec3::new(-3.2, -3.2, 0.0),
    );

    let title_id = scene.add_tattva(
        Label::new("Milestone 4 STEM Showcase", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
