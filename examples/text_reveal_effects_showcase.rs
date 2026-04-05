/// Demonstration of both typewriter and reveal text effects
/// Shows the difference between fixed-position and shifting text animations
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

    // ===== TYPEWRITER MODE (Fixed Position) =====
    let mut typewriter_label = Label::new("Typewriter Effect", 0.5)
        .with_color(Vec4::new(0.19, 0.64, 0.33, 1.0));
    typewriter_label.typewriter_mode = true;  // Explicitly set typewriter mode
    let typewriter_id = scene.add_tattva(typewriter_label, Vec3::new(-4.0, 2.0, 0.0));

    // let typewriter_desc = Label::new("Text grows from left to right", 0.3)
    //     .with_color(Vec4::new(0.19, 0.64, 0.33, 0.7));
    // let typewriter_desc_id = scene.add_tattva(typewriter_desc, Vec3::new(-4.0, 1.2, 0.0));

    // ===== REVEAL MODE (Shifting/Growing from Center) =====
    let reveal_label = Label::new("Reveal Effect", 0.5)
        .with_color(Vec4::new(0.92, 0.26, 0.21, 1.0));
    let reveal_id = scene.add_tattva(reveal_label, Vec3::new(2.0, 2.0, 0.0));

    // let reveal_desc = Label::new("Text grows from center", 0.3)
    //     .with_color(Vec4::new(0.92, 0.26, 0.21, 0.7));
    // let reveal_desc_id = scene.add_tattva(reveal_desc, Vec3::new(2.0, 1.2, 0.0));

    // // ===== LATEX EXAMPLES =====
    // let typewriter_latex = Latex::new("f(x) = x^2", 0.4)
    //     .with_color(Vec4::new(0.16, 0.50, 0.73, 1.0));
    // let typewriter_latex_id = scene.add_tattva(typewriter_latex, Vec3::new(-4.0, -0.5, 0.0));

    // let reveal_latex = Latex::new("g(x) = 2x + 1", 0.4)
    //     .with_color(Vec4::new(0.96, 0.80, 0.19, 1.0));
    // let reveal_latex_id = scene.add_tattva(reveal_latex, Vec3::new(2.0, -0.5, 0.0));

    scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

    // ===== TYPEWRITER ANIMATIONS =====
    // Write typewriter label
    timeline.animate(typewriter_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .write_text()
        .spawn();

    // Write typewriter description
    // timeline.animate(typewriter_desc_id)
    //     .at(0.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .write_text()
    //     .spawn();

    // Write typewriter LaTeX
    // timeline.animate(typewriter_latex_id)
    //     .at(1.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .write_text()
    //     .spawn();

    // ===== REVEAL ANIMATIONS =====
    // Reveal label (grows from center)
    timeline.animate(reveal_id)
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
    // Unwrite typewriter label
    timeline.animate(typewriter_id)
        .at(4.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .unwrite_text()
        .spawn();

    // Unwrite typewriter description
    // timeline.animate(typewriter_desc_id)
    //     .at(4.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .unwrite_text()
    //     .spawn();

    // // Unwrite typewriter LaTeX
    // timeline.animate(typewriter_latex_id)
    //     .at(5.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .unwrite_text()
    //     .spawn();

    // ===== UNREVEAL ANIMATIONS =====
    // Unreveal label (shrinks to center)
    timeline.animate(reveal_id)
        .at(4.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .unreveal_text()
        .spawn();

    // Unreveal description
    // timeline.animate(reveal_desc_id)
    //     .at(4.5)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .unreveal_text()
    //     .spawn();

    // // Unreveal LaTeX
    // timeline.animate(reveal_latex_id)
    //     .at(5.0)
    //     .for_duration(1.5)
    //     .ease(Ease::Linear)
    //     .unreveal_text()
    //     .spawn();

    scene.timelines.insert("main".to_string(), timeline);

    App::new()?.with_scene(scene).run_app()
}
