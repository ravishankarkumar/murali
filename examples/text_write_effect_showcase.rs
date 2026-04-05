/// Demonstration of write/unwrite effects for text
/// Shows typewriter-style character reveal and hide animations
use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::text::latex::Latex;
use murali::frontend::animation::Ease;
use murali::engine::timeline::Timeline;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // ===== LABEL TEXT (Simple text) =====
    let label1 = Label::new("Hello Murali!", 0.5)
        .with_color(Vec4::new(0.19, 0.64, 0.33, 1.0));
    let label1_id = scene.add_tattva(label1, Vec3::new(-3.0, 2.0, 0.0));

    let label2 = Label::new("Write and Unwrite Effects", 0.4)
        .with_color(Vec4::new(0.92, 0.26, 0.21, 1.0));
    let label2_id = scene.add_tattva(label2, Vec3::new(-3.0, 1.0, 0.0));

    let label3 = Label::new("Character by character reveal", 0.35)
        .with_color(Vec4::new(0.16, 0.50, 0.73, 1.0));
    let label3_id = scene.add_tattva(label3, Vec3::new(-3.0, 0.0, 0.0));

    // ===== LATEX TEXT =====
    let latex1 = Latex::new("f(x) = x^2 + 2x + 1", 0.5)
        .with_color(Vec4::new(0.96, 0.80, 0.19, 1.0));
    let latex1_id = scene.add_tattva(latex1, Vec3::new(2.0, 2.0, 0.0));

    let latex2 = Latex::new("\\int_0^\\infty e^{-x} dx = 1", 0.4)
        .with_color(Vec4::new(0.61, 0.35, 0.71, 1.0));
    let latex2_id = scene.add_tattva(latex2, Vec3::new(2.0, 0.5, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    // ===== WRITE ANIMATIONS =====
    // Label 1: Write effect
    timeline.animate(label1_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    // Label 2: Write effect (delayed)
    timeline.animate(label2_id)
        .at(0.5)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    // Label 3: Write effect (delayed more)
    timeline.animate(label3_id)
        .at(1.0)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    // LaTeX 1: Write effect
    timeline.animate(latex1_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    // LaTeX 2: Write effect (delayed)
    timeline.animate(latex2_id)
        .at(0.5)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    // ===== UNWRITE ANIMATIONS =====
    // Label 1: Unwrite effect
    timeline.animate(label1_id)
        .at(5.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .unwrite_text()
        .spawn();

    // Label 2: Unwrite effect (delayed)
    timeline.animate(label2_id)
        .at(5.5)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .unwrite_text()
        .spawn();

    // Label 3: Unwrite effect (delayed more)
    timeline.animate(label3_id)
        .at(6.0)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .unwrite_text()
        .spawn();

    // LaTeX 1: Unwrite effect
    timeline.animate(latex1_id)
        .at(5.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .unwrite_text()
        .spawn();

    // LaTeX 2: Unwrite effect (delayed)
    timeline.animate(latex2_id)
        .at(5.5)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .unwrite_text()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
