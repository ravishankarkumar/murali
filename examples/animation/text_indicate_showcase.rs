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

    let title_id = scene.add_tattva(
        Label::new("Text Indicate Showcase", 0.40).with_color(WHITE),
        Vec3::new(0.0, 2.6, 0.0),
    );

    let label_id = scene.add_tattva(
        Label::new("AI Agent uses LLM and takes autonomous decisions", 0.34)
            .with_color(GREEN_D),
        Vec3::new(0.0, 0.8, 0.0),
    );

    let caption_id = scene.add_tattva(
        Label::new("Indicate gently pulses and brightens text.", 0.22)
            .with_color(GRAY_A),
        Vec3::new(0.0, -0.6, 0.0),
    );

    // let latex_id = scene.add_tattva(
    //     Latex::new(r"\nabla \cdot \mathbf{F} = \rho", 0.52)
    //         .with_color(GOLD_C),
    //     Vec3::new(0.0, -2.1, 0.0),
    // );

    timeline
        .animate(title_id)
        .at(0.4)
        .for_duration(0.8)
        .ease(Ease::InOutCubic)
        .indicate()
        .spawn();

    timeline
        .animate(label_id)
        .at(1.3)
        .for_duration(1.0)
        .ease(Ease::InOutCubic)
        .indicate()
        .spawn();

    timeline
        .animate(caption_id)
        .at(2.5)
        .for_duration(0.8)
        .ease(Ease::InOutCubic)
        .indicate()
        .spawn();

    // timeline
    //     .animate(latex_id)
    //     .at(3.4)
    //     .for_duration(1.0)
    //     .ease(Ease::InOutCubic)
    //     .indicate()
    //     .spawn();

    timeline
        .animate(label_id)
        .at(4.8)
        .for_duration(0.9)
        .ease(Ease::InOutCubic)
        .indicate()
        .spawn();

    // timeline
    //     .animate(latex_id)
    //     .at(5.2)
    //     .for_duration(0.9)
    //     .ease(Ease::InOutCubic)
    //     .indicate()
    //     .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    App::new()?.with_scene(scene).run_app()
}
