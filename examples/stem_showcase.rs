use glam::{vec2, Vec2, Vec3, Vec4};
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
use murali::frontend::Tattva;
use murali::App;

fn add_tattva<T>(scene: &mut Scene, state: T, position: Vec3) -> usize
where
    T: murali::projection::Project + murali::frontend::layout::Bounded + Send + Sync + 'static,
{
    let tattva = Tattva::new(0, state);
    let id = scene.add(tattva);

    if let Some(t) = scene.get_tattva_any_mut(id) {
        let mut props = t.props().write();
        props.position = position;
    }

    id
}

fn parabola(x: f32) -> f32 {
    0.18 * x * x - 1.2
}

fn spiral(t: f32) -> Vec2 {
    let r = 0.15 * t;
    vec2(r * t.cos(), r * t.sin())
}

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    add_tattva(
        &mut scene,
        NumberPlane::new((-5.0, 5.0), (-3.0, 3.0)).with_step(1.0),
        Vec3::new(-3.2, 0.2, 0.0),
    );

    add_tattva(
        &mut scene,
        FunctionGraph::new((-4.5, 4.5), parabola).with_samples(160),
        Vec3::new(-3.2, 0.2, 0.0),
    );

    add_tattva(
        &mut scene,
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

    add_tattva(
        &mut scene,
        ParametricCurve::new((0.0, 10.0 * std::f32::consts::PI), spiral).with_samples(220),
        Vec3::new(2.2, 0.6, 0.0),
    );

    add_tattva(
        &mut scene,
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

    add_tattva(
        &mut scene,
        EquationLayout::new(
            vec![
                EquationPart::new("y"),
                EquationPart::new("=").with_color(Vec4::new(0.95, 0.88, 0.42, 1.0)),
                EquationPart::new("0.18x^2").with_color(Vec4::new(0.38, 0.80, 0.97, 1.0)),
                EquationPart::new("-"),
                EquationPart::new("1.2").with_color(Vec4::new(0.96, 0.54, 0.29, 1.0)),
            ],
            0.30,
        ),
        Vec3::new(-3.2, -3.2, 0.0),
    );

    let title_id = add_tattva(
        &mut scene,
        Label::new("Milestone 4 STEM Showcase", 0.36)
            .with_color(Vec4::new(0.97, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 11.0);

    App::new()?.with_scene(scene).run_app()
}
