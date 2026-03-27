use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::text::typst::Typst;
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
        Label::new("Typst Showcase", 0.34).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::new(0.0, 0.0, 0.0),
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    add_tattva(
        &mut scene,
        Typst::new(
            r#"$f(x) = x^2 + 2 x + 1$"#,
            0.44,
        )
        .with_color(Vec4::new(0.98, 0.88, 0.38, 1.0)),
        Vec3::new(0.0, 1.55, 0.0),
    );

    add_tattva(
        &mut scene,
        Typst::new(
            r#"#align(center)[
              #text(weight: "semibold", 14pt)[Transformer Attention]
              #v(0.4em)
              $A(Q, K, V) = op("softmax")(Q K^T / sqrt(d_k)) V$
            ]"#,
            0.54,
        )
        .with_color(Vec4::new(0.37, 0.80, 0.97, 1.0)),
        Vec3::new(0.0, -0.15, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("Use this scene to validate Typst compile, raster, and layout.", 0.22)
            .with_color(Vec4::new(0.78, 0.82, 0.87, 1.0)),
        Vec3::new(0.0, -3.0, 0.0),
    );

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);
    App::new()?.with_scene(scene).run_app()
}
