use glam::{Vec3, Vec4};
use murali::App;
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
        Label::new("Perlin Noise Circle", 0.36).with_color(Vec4::new(0.97, 0.98, 0.99, 1.0)),
        Vec3::new(0.0, 3.15, 0.0),
    );

    let circle_id = scene.add_tattva(
        PerlinNoiseCircle::new(1.8, Vec4::new(0.98, 0.74, 0.28, 1.0))
            .with_samples(220)
            .with_noise_frequency(1.6)
            .with_noise_amplitude(0.28)
            .with_noise_seed(1.37)
            .with_phase(0.0)
            .with_morph_speed(1.1)
            .with_gradient(
                PerlinNoiseCircleGradient::new(vec![
                    Vec4::new(0.20, 0.92, 1.00, 1.0),
                    Vec4::new(0.49, 0.58, 1.00, 1.0),
                    Vec4::new(0.98, 0.42, 0.86, 1.0),
                    Vec4::new(0.98, 0.78, 0.30, 1.0),
                ])
                .with_cycles(2.3)
                .with_motion_rate(0.42),
            )
            .with_stroke(0.055, Vec4::new(0.98, 0.74, 0.28, 1.0)),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Label::new(
            "Polar Perlin noise keeps the contour closed while shape and colors evolve together.",
            0.20,
        )
        .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
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

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 8.0);

    App::new()?.with_scene(scene).run_app()
}
