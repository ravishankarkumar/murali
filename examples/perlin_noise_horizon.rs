use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::noisy_horizon::{
    MultiLayeredPerlinField, NoisyHorizonGradient, PerlinFieldLayer,
};
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("AI Perlin Field", 0.34).with_color(Vec4::new(0.97, 0.98, 0.99, 1.0)),
        Vec3::new(0.0, 3.2, 0.0),
    );

    let field_id = scene.add_tattva(
        MultiLayeredPerlinField::new(13.0)
            .with_x_range(-6.5, 6.5)
            .with_baseline_y(-1.4)
            .with_bottom_y(-4.4)
            .with_samples(280)
            .with_noise_frequency(0.21)
            .with_noise_amplitude(0.62)
            .with_noise_seed(2.15)
            .with_morph_speed(0.78)
            .with_gradient(
                NoisyHorizonGradient::new(vec![
                    Vec4::new(0.06, 0.82, 0.98, 0.98),
                    Vec4::new(0.30, 0.57, 1.00, 0.94),
                    Vec4::new(0.72, 0.37, 0.98, 0.92),
                    Vec4::new(0.98, 0.66, 0.34, 0.90),
                    Vec4::new(0.88, 0.22, 0.54, 0.88),
                ])
                .with_cycles(1.45)
                .with_motion_rate(0.21)
                .with_vertical_shift(0.32),
            )
            .with_layer_count(7)
            .with_layers(vec![
                PerlinFieldLayer::new(0.0)
                    .with_amplitude_scale(1.0)
                    .with_frequency_scale(1.0)
                    .with_phase_offset(0.0)
                    .with_opacity(0.14)
                    .with_stroke_thickness(0.018),
                PerlinFieldLayer::new(-0.14)
                    .with_amplitude_scale(0.96)
                    .with_frequency_scale(1.04)
                    .with_phase_offset(0.55)
                    .with_opacity(0.12)
                    .with_stroke_thickness(0.017),
                PerlinFieldLayer::new(-0.28)
                    .with_amplitude_scale(0.92)
                    .with_frequency_scale(1.09)
                    .with_phase_offset(1.05)
                    .with_opacity(0.11)
                    .with_stroke_thickness(0.016),
                PerlinFieldLayer::new(-0.42)
                    .with_amplitude_scale(0.86)
                    .with_frequency_scale(1.15)
                    .with_phase_offset(1.55)
                    .with_opacity(0.10)
                    .with_stroke_thickness(0.015),
                PerlinFieldLayer::new(-0.56)
                    .with_amplitude_scale(0.80)
                    .with_frequency_scale(1.22)
                    .with_phase_offset(2.05)
                    .with_opacity(0.09)
                    .with_stroke_thickness(0.014),
                PerlinFieldLayer::new(-0.70)
                    .with_amplitude_scale(0.74)
                    .with_frequency_scale(1.30)
                    .with_phase_offset(2.60)
                    .with_opacity(0.08)
                    .with_stroke_thickness(0.013),
                PerlinFieldLayer::new(-0.84)
                    .with_amplitude_scale(0.68)
                    .with_frequency_scale(1.39)
                    .with_phase_offset(3.10)
                    .with_opacity(0.07)
                    .with_stroke_thickness(0.012),
            ])
            .with_stroke_color(Vec4::new(0.96, 0.98, 1.0, 0.20)),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Label::new(
            "Layered translucent Perlin waves create a sleek AI-processing field.",
            0.19,
        )
        .with_color(Vec4::new(0.78, 0.83, 0.89, 1.0)),
        Vec3::new(0.0, -3.5, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(field_id)
        .at(0.0)
        .for_duration(8.0)
        .ease(Ease::Linear)
        .perlin_field_evolve()
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 8.0);

    App::new()?.with_scene(scene).run_app()
}
