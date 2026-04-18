use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::particle_belt::AsteroidBelt;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Particle Belt", 0.36).with_color(WHITE),
        Vec3::new(0.0, 3.25, 0.0),
    );

    let belt_id = scene.add_tattva(
        AsteroidBelt::new(2.0)
            .with_band_width(0.9)
            .with_particle_count(220)
            .with_particle_size_range(0.012, 0.05)
            .with_palette(vec![
                BLUE_A,
                BLUE_B,
                PINK_B,
                GOLD_C,
                GREEN_B,
            ])
            .with_orbit_speed(0.85)
            .with_clockwise_ratio(0.9)
            .with_band_breathing(0.11, 1.15)
            .with_radial_jitter(0.13, 2.5)
            .with_seed(2.4),
        Vec3::ZERO,
    );

    scene.add_tattva(
        Label::new(
            "A living orbital band with gentle radius breathing and 90% clockwise motion.",
            0.20,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, -3.2, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(belt_id)
        .at(0.0)
        .for_duration(8.0)
        .ease(Ease::Linear)
        .belt_evolve()
        .spawn();

    scene.set_timeline("main", timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 8.5);

    App::new()?.with_scene(scene).run_app()
}
