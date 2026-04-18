/// Demonstration of typewriter-style text animations
/// Shows fixed-anchor character-by-character reveal and hide animations
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::text::latex::Latex;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    scene.add_tattva(
        Label::new("Typewriter Text", 0.34).with_color(WHITE),
        Vec3::new(0.0, 3.1, 0.0),
    );

    // Subtitle
    scene.add_tattva(
        Label::new("Fixed-anchor text and LaTeX animation", 0.18)
            .with_color(GRAY_B),
        Vec3::new(0.0, 2.55, 0.0),
    );

    // Content
    let label1 = Label::new("Hello Murali!", 0.5).with_color(GREEN_D);
    let label1_id = scene.add_tattva(label1, Vec3::new(-3.0, 2.0, 0.0));

    let label2 =
        Label::new("Write and Unwrite Effects", 0.4).with_color(RED_B);
    let label2_id = scene.add_tattva(label2, Vec3::new(-3.0, 1.0, 0.0));

    let label3 = Label::new("Character by character reveal", 0.35)
        .with_color(BLUE_D);
    let label3_id = scene.add_tattva(label3, 3.0 * LEFT);

    // ===== LATEX TEXT =====
    let latex1 =
        Latex::new("f(x) = x^2 + 2x + 1", 0.5).with_color(YELLOW_B);
    let latex1_id = scene.add_tattva(latex1, 2.0 * RIGHT + 2.0 * UP);

    let latex2 = Latex::new("\\int_0^\\infty e^{-x} dx = 1", 0.4)
        .with_color(PURPLE_B);
    let latex2_id = scene.add_tattva(latex2, Vec3::new(2.0, 0.5, 0.0));

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // Timeline
    let mut timeline = Timeline::new();

    timeline
        .animate(label1_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(label2_id)
        .at(0.5)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(label3_id)
        .at(1.0)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(latex1_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(latex2_id)
        .at(0.5)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    timeline
        .animate(label1_id)
        .at(5.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    timeline
        .animate(label2_id)
        .at(5.5)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    timeline
        .animate(label3_id)
        .at(6.0)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    timeline
        .animate(latex1_id)
        .at(5.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    timeline
        .animate(latex2_id)
        .at(5.5)
        .for_duration(2.5)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}
