use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::particle_belt::AsteroidBelt;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    scene.add_tattva(
        Label::new("Particle Belt", 0.36).with_color(Vec4::new(0.97, 0.98, 0.99, 1.0)),
        Vec3::new(0.0, 3.25, 0.0),
    );

    let belt_id = scene.add_tattva(
        AsteroidBelt::new(2.0)
            .with_band_width(0.9)
            .with_particle_count(220)
            .with_particle_size_range(0.012, 0.05)
            .with_palette(vec![
                Vec4::new(0.22, 0.92, 1.00, 1.0),
                Vec4::new(0.58, 0.62, 1.00, 1.0),
                Vec4::new(0.97, 0.44, 0.81, 1.0),
                Vec4::new(1.00, 0.78, 0.33, 1.0),
                Vec4::new(0.76, 0.98, 0.62, 1.0),
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
        .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
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

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 8.5);

    App::new()?.with_scene(scene).run_app()
}
