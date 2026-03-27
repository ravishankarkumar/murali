use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::text::latex::Latex;
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

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    add_tattva(
        &mut scene,
        Label::new("LaTeX Showcase", 0.34).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    add_tattva(
        &mut scene,
        Latex::new(r"\int_0^1 x^2 \, dx = \frac{1}{3}", 0.72)
            .with_color(Vec4::new(0.98, 0.88, 0.38, 1.0)),
        Vec3::new(0.0, 1.35, 0.0),
    );

    add_tattva(
        &mut scene,
        Latex::new(r"E = mc^2", 0.60)
            .with_color(Vec4::new(0.38, 0.80, 0.97, 1.0)),
        Vec3::new(-3.2, -0.2, 0.0),
    );

    add_tattva(
        &mut scene,
        Latex::new(r"\mathbf{A} = \begin{bmatrix} 1 & 2 \\ 3 & 4 \end{bmatrix}", 0.78)
            .with_color(Vec4::new(0.92, 0.94, 0.97, 1.0)),
        Vec3::new(2.35, -0.35, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("Use this scene to validate LaTeX compile, raster, and sizing.", 0.22)
            .with_color(Vec4::new(0.78, 0.82, 0.87, 1.0)),
        Vec3::new(0.0, -3.0, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);
    App::new()?.with_scene(scene).run_app()
}
