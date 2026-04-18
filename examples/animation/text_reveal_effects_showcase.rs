/// Demonstration of both typewriter and centered reveal text effects
/// Shows the difference between fixed-anchor and shifting text animations
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // ===== TYPEWRITER MODE (Fixed Position) =====
    let mut typewriter_label =
        Label::new("Typewriter Effect", 0.5).with_color(GREEN_D);
    typewriter_label.typewriter_mode = true; // Explicitly set typewriter mode
    let typewriter_id = scene.add_tattva(typewriter_label, 4.0 * LEFT + 2.0 * UP);

    // let typewriter_desc = Label::new("Text grows from left to right", 0.3)
    //     .with_color(GREEN_D);
    // let typewriter_desc_id = scene.add_tattva(typewriter_desc, 4.0 * LEFT + 1.2 * UP);

    // ===== REVEAL MODE (Shifting/Growing from Center) =====
    let reveal_label =
        Label::new("Reveal Effect", 0.5).with_color(RED_B);
    let reveal_id = scene.add_tattva(reveal_label, 2.0 * RIGHT + 2.0 * UP);

    // let reveal_desc = Label::new("Text grows from center", 0.3)
    //     .with_color(RED_B);
    // let reveal_desc_id = scene.add_tattva(reveal_desc, 2.0 * RIGHT + 1.2 * UP);

    // // ===== LATEX EXAMPLES =====
    // let typewriter_latex = Latex::new("f(x) = x^2", 0.4)
    //     .with_color(BLUE_D);
    // let typewriter_latex_id = scene.add_tattva(typewriter_latex, 4.0 * LEFT + 0.5 * DOWN);

    // let reveal_latex = Latex::new("g(x) = 2x + 1", 0.4)
    //     .with_color(YELLOW_B);
    // let reveal_latex_id = scene.add_tattva(reveal_latex, 2.0 * RIGHT + 0.5 * DOWN);

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // ===== TYPEWRITER ANIMATIONS =====
    // Typewrite label
    timeline
        .animate(typewriter_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    // Write typewriter description
    // timeline.animate(typewriter_desc_id)
    //     .at(0.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .typewrite_text()
    //     .spawn();

    // Write typewriter LaTeX
    // timeline.animate(typewriter_latex_id)
    //     .at(1.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .typewrite_text()
    //     .spawn();

    // ===== REVEAL ANIMATIONS =====
    // Reveal label (grows from center)
    timeline
        .animate(reveal_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .reveal_text()
        .spawn();

    // Reveal description
    // timeline.animate(reveal_desc_id)
    //     .at(0.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .reveal_text()
    //     .spawn();

    // // Reveal LaTeX
    // timeline.animate(reveal_latex_id)
    //     .at(1.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .reveal_text()
    //     .spawn();

    // ===== UNWRITE ANIMATIONS =====
    // Reverse typewriter label
    timeline
        .animate(typewriter_id)
        .at(4.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    // Unwrite typewriter description
    // timeline.animate(typewriter_desc_id)
    //     .at(4.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .untypewrite_text()
    //     .spawn();

    // // Unwrite typewriter LaTeX
    // timeline.animate(typewriter_latex_id)
    //     .at(5.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .untypewrite_text()
    //     .spawn();

    // ===== UNREVEAL ANIMATIONS =====
    // Hide revealed label (shrinks to center)
    timeline
        .animate(reveal_id)
        .at(4.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .hide_text()
        .spawn();

    // Unreveal description
    // timeline.animate(reveal_desc_id)
    //     .at(4.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .hide_text()
    //     .spawn();

    // // Unreveal LaTeX
    // timeline.animate(reveal_latex_id)
    //     .at(5.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .hide_text()
    //     .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}
