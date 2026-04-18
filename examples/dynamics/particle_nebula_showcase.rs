use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::particle_belt::ParticleBelt;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Particle Nebula Showcase", 0.36).with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new(
            "Layered particle belts form a bright core, drifting arcs, and a halo of cosmic dust.",
            0.18,
        )
        .with_color(GRAY_B),
        Vec3::new(0.0, 3.12, 0.0),
    );

    // Content
    let glow_outer_id = scene.add_tattva(
        Circle::new(0.95, 48, Vec4::new(BLUE_D.x, BLUE_D.y, BLUE_D.z, 0.18)),
        Vec3::new(0.0, 0.1, 0.0),
    );
    let glow_mid_id = scene.add_tattva(
        Circle::new(0.55, 48, Vec4::new(PURPLE_B.x, PURPLE_B.y, PURPLE_B.z, 0.26)),
        Vec3::new(0.0, 0.1, 0.0),
    );
    let core_id = scene.add_tattva(
        Circle::new(0.18, 32, GOLD_A)
            .with_stroke(0.03, ORANGE_D),
        Vec3::new(0.0, 0.1, 0.0),
    );

    let core_halo_id = scene.add_tattva(
        ParticleBelt::new(1.1)
            .with_band_width(0.45)
            .with_particle_count(180)
            .with_particle_size_range(0.016, 0.055)
            .with_palette(vec![
                BLUE_A,
                PURPLE_B,
                PINK_B,
                GOLD_C,
            ])
            .with_orbit_speed(1.15)
            .with_clockwise_ratio(0.82)
            .with_band_breathing(0.10, 1.35)
            .with_radial_jitter(0.12, 2.7)
            .with_seed(2.1),
        Vec3::new(0.0, 0.1, 0.0),
    );

    let inner_arc_id = scene.add_tattva(
        ParticleBelt::new(2.4)
            .with_band_width(0.65)
            .with_particle_count(260)
            .with_particle_size_range(0.010, 0.038)
            .with_palette(vec![
                TEAL_B,
                BLUE_B,
                PURPLE_B,
            ])
            .with_orbit_speed(0.92)
            .with_clockwise_ratio(1.0)
            .with_band_breathing(0.16, 0.9)
            .with_radial_jitter(0.18, 1.9)
            .with_angular_spread(std::f32::consts::PI * 1.25)
            .with_seed(5.2),
        Vec3::new(-0.15, 0.05, 0.0),
    );

    let outer_arc_id = scene.add_tattva(
        ParticleBelt::new(3.7)
            .with_band_width(0.95)
            .with_particle_count(340)
            .with_particle_size_range(0.008, 0.032)
            .with_palette(vec![
                ORANGE_C,
                GOLD_C,
                WHITE,
            ])
            .with_orbit_speed(0.62)
            .with_clockwise_ratio(0.0)
            .with_band_breathing(0.22, 0.7)
            .with_radial_jitter(0.24, 1.45)
            .with_angular_spread(std::f32::consts::PI * 1.55)
            .with_seed(8.4),
        Vec3::new(0.35, -0.05, 0.0),
    );

    let dust_shell_id = scene.add_tattva(
        ParticleBelt::new(5.0)
            .with_band_width(1.45)
            .with_particle_count(520)
            .with_particle_size_range(0.004, 0.020)
            .with_palette(vec![
                BLUE_A,
                PURPLE_B,
                GOLD_C,
                WHITE,
            ])
            .with_orbit_speed(0.34)
            .with_clockwise_ratio(0.58)
            .with_band_breathing(0.18, 0.55)
            .with_radial_jitter(0.25, 1.1)
            .with_seed(12.7),
        ORIGIN,
    );

    scene.set_opacity(glow_outer_id, 0.0);
    scene.set_opacity(glow_mid_id, 0.0);
    scene.set_opacity(core_id, 0.0);
    scene.set_opacity(core_halo_id, 0.0);
    scene.set_opacity(inner_arc_id, 0.0);
    scene.set_opacity(outer_arc_id, 0.0);
    scene.set_opacity(dust_shell_id, 0.0);

    scene.set_scale(glow_outer_id, Vec3::splat(0.7));
    scene.set_scale(glow_mid_id, Vec3::splat(0.75));
    scene.set_scale(core_halo_id, Vec3::splat(0.88));
    scene.set_scale(inner_arc_id, Vec3::new(0.82, 0.82, 1.0));
    scene.set_scale(outer_arc_id, Vec3::new(0.86, 0.86, 1.0));
    scene.set_scale(dust_shell_id, Vec3::new(0.92, 0.92, 1.0));

    // Camera
    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene.camera_mut().set_view_width(16.0);

    // Timeline
    let mut timeline = Timeline::new();

    timeline
        .animate(glow_outer_id)
        .at(0.0)
        .for_duration(1.4)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
    timeline
        .animate(glow_outer_id)
        .at(0.0)
        .for_duration(1.8)
        .ease(Ease::OutCubic)
        .scale_to(Vec3::splat(1.0))
        .spawn();

    timeline
        .animate(glow_mid_id)
        .at(0.2)
        .for_duration(1.2)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
    timeline
        .animate(glow_mid_id)
        .at(0.2)
        .for_duration(1.6)
        .ease(Ease::OutCubic)
        .scale_to(Vec3::splat(1.0))
        .spawn();

    timeline
        .animate(core_id)
        .at(0.35)
        .for_duration(0.8)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();

    timeline
        .animate(core_halo_id)
        .at(0.5)
        .for_duration(1.5)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
    timeline
        .animate(core_halo_id)
        .at(0.5)
        .for_duration(1.7)
        .ease(Ease::OutCubic)
        .scale_to(Vec3::splat(1.0))
        .spawn();

    timeline
        .animate(inner_arc_id)
        .at(1.0)
        .for_duration(1.8)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
    timeline
        .animate(inner_arc_id)
        .at(1.0)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .scale_to(Vec3::ONE)
        .spawn();

    timeline
        .animate(outer_arc_id)
        .at(1.8)
        .for_duration(2.0)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
    timeline
        .animate(outer_arc_id)
        .at(1.8)
        .for_duration(2.2)
        .ease(Ease::OutCubic)
        .scale_to(Vec3::ONE)
        .spawn();

    timeline
        .animate(dust_shell_id)
        .at(2.5)
        .for_duration(2.3)
        .ease(Ease::OutCubic)
        .appear()
        .spawn();
    timeline
        .animate(dust_shell_id)
        .at(2.5)
        .for_duration(2.6)
        .ease(Ease::OutCubic)
        .scale_to(Vec3::ONE)
        .spawn();

    timeline
        .animate(core_halo_id)
        .at(0.0)
        .for_duration(24.0)
        .ease(Ease::Linear)
        .belt_evolve_with_speed(1.15)
        .spawn();
    timeline
        .animate(inner_arc_id)
        .at(0.0)
        .for_duration(24.0)
        .ease(Ease::Linear)
        .belt_evolve_with_speed(0.92)
        .spawn();
    timeline
        .animate(outer_arc_id)
        .at(0.0)
        .for_duration(24.0)
        .ease(Ease::Linear)
        .belt_evolve_with_speed(0.62)
        .spawn();
    timeline
        .animate(dust_shell_id)
        .at(0.0)
        .for_duration(24.0)
        .ease(Ease::Linear)
        .belt_evolve_with_speed(0.34)
        .spawn();

    timeline
        .animate(glow_outer_id)
        .at(6.0)
        .for_duration(5.0)
        .ease(Ease::InOutQuad)
        .fade_to(0.08)
        .spawn();
    timeline
        .animate(glow_outer_id)
        .at(11.0)
        .for_duration(5.0)
        .ease(Ease::InOutQuad)
        .fade_to(0.18)
        .spawn();

    timeline
        .animate(glow_mid_id)
        .at(5.0)
        .for_duration(4.5)
        .ease(Ease::InOutQuad)
        .fade_to(0.12)
        .spawn();
    timeline
        .animate(glow_mid_id)
        .at(9.5)
        .for_duration(4.5)
        .ease(Ease::InOutQuad)
        .fade_to(0.26)
        .spawn();

    timeline
        .animate_camera()
        .at(0.0)
        .for_duration(8.0)
        .ease(Ease::InOutQuad)
        .frame_to(Vec3::new(-0.45, 0.25, 9.2), Vec3::new(0.0, 0.1, 0.0))
        .spawn();
    timeline
        .animate_camera()
        .at(8.0)
        .for_duration(8.0)
        .ease(Ease::InOutQuad)
        .frame_to(Vec3::new(0.55, -0.18, 8.8), Vec3::new(0.0, 0.05, 0.0))
        .spawn();
    timeline
        .animate_camera()
        .at(16.0)
        .for_duration(8.0)
        .ease(Ease::InOutQuad)
        .frame_to(Vec3::new(0.0, 0.0, 10.2), Vec3::new(0.0, 0.1, 0.0))
        .spawn();

    scene.play(timeline);

    // Final app run
    App::new()?.with_scene(scene).run_app()
}
