use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::text::typst::Typst;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Typst Showcase", 0.34).with_color(WHITE),
        ORIGIN,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    scene.add_tattva(
        Typst::new(r#"$f(x) = x^2 + 2 x + 1$"#, 0.44).with_color(GOLD_B),
        Vec3::new(0.0, 1.55, 0.0),
    );

    scene.add_tattva(
        Typst::new(
            r#"#align(center)[
              #text(weight: "semibold", 14pt)[Transformer Attention]
              #v(0.4em)
              $A(Q, K, V) = op("softmax")(Q K^T / sqrt(d_k)) V$
            ]"#,
            0.54,
        )
        .with_color(BLUE_B),
        Vec3::new(0.0, -0.15, 0.0),
    );

    scene.add_tattva(
        Label::new(
            "Use this scene to validate Typst compile, raster, and layout.",
            0.22,
        )
        .with_color(GRAY_A),
        3.0 * DOWN,
    );

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    App::new()?.with_scene(scene).run_app()
}
