use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::noisy_circle::{
    PerlinNoiseCircle, PerlinNoiseCircleGradient,
};
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Perlin Noise Circle", 0.36).with_color(WHITE),
        3.15 * UP,
    );

    let circle_id = scene.add_tattva(
        PerlinNoiseCircle::new(1.8, GOLD_C)
            .with_samples(220)
            .with_noise_frequency(1.6)
            .with_noise_amplitude(0.28)
            .with_noise_seed(1.37)
            .with_phase(0.0)
            .with_morph_speed(1.1)
            .with_gradient(
                PerlinNoiseCircleGradient::new(vec![
                    BLUE_A,
                    BLUE_A,
                    PINK_B,
                    GOLD_C,
                ])
                .with_cycles(2.3)
                .with_motion_rate(0.42),
            )
            .with_stroke(0.055, GOLD_C),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Label::new(
            "Polar Perlin noise keeps the contour closed while shape and colors evolve together.",
            0.20,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -3.2, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(circle_id)
        .at(0.0)
        .for_duration(6.0)
        .ease(Ease::Linear)
        .noise_evolve()
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 8.0);

    App::new()?.with_scene(scene).run_app()
}
