/// Force Field Example with Updaters
/// Demonstrates a moving charged particle affecting a grid of force vectors
/// Each vector updates based on the inverse square law

use glam::{vec3, Quat, Vec2, Vec3, Vec4};
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::arrow::Arrow;
use murali::frontend::collection::primitives::circle::Circle;
use murali::App;
use std::f32::consts::PI;
use murali::engine::timeline::Timeline;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Create the charged particle (positive charge - red)
    let particle_radius: f32 = 0.2;
    let particle = Circle::new(particle_radius, 24, RED_C).with_stroke(0.04, RED_B);

    let particle_id = scene.add_tattva(particle, Vec3::new(-5.0, 0.0, 0.0));

    // Create a grid of force field vectors spanning the full screen
    let grid_spacing: f32 = 0.8;

    // Canonical Murali world bounds
    let x_min: f32 = -8.0;
    let x_max: f32 = 8.0;
    let y_min: f32 = -4.5;
    let y_max: f32 = 4.5;

    let nx = ((x_max - x_min) / grid_spacing).ceil() as i32;
    let ny = ((y_max - y_min) / grid_spacing).ceil() as i32;

    let mut vector_ids = Vec::new();

    for iy in 0..=ny {
        for ix in 0..=nx {
            let x = x_min + ix as f32 * grid_spacing;
            let y = y_min + iy as f32 * grid_spacing;
            let pos = vec3(x, y, 0.0);

            let arrow = Arrow::with_default_tip(
                Vec2::ZERO,
                Vec2::new(0.0, 0.3), // Initial direction (will be updated)
                0.03,
                Vec4::new(BLUE_B.x, BLUE_B.y, BLUE_B.z, 0.8),
            );

            let vector_id = scene.add_tattva(arrow, pos);
            vector_ids.push((vector_id, pos));
        }
    }

    // Animate the charged particle moving in a circle
    let duration: f32 = 8.0;
    let radius: f32 = 3.0;
    let start_time = scene.scene_time;

    scene.add_updater(particle_id, move |scene, pid, _dt| {
        let t = scene.scene_time - start_time;
        let angle = (t / duration) * 2.0 * PI;

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        scene.set_position_2d(pid, Vec2::new(x, y));
    });

    // Add updaters to each force vector to respond to the charged particle
    let charge_strength: f32 = 2.0;

    for (vector_id, vector_pos) in vector_ids {
        scene.add_updater(vector_id, move |scene, vid, _dt| {
            if let Some(particle_tattva) = scene.get_tattva_any(particle_id) {
                let p_props =
                    murali::frontend::props::DrawableProps::read(particle_tattva.props());
                let particle_pos = p_props.position;
                drop(p_props);

                // Force direction at this grid point
                // F = k * q / r^2, direction away from positive charge
                let delta = vector_pos - particle_pos;
                let distance = delta.length();

                // Avoid division by zero and cap the visual magnitude
                let safe_distance = distance.max(0.5);
                let force_magnitude = charge_strength / (safe_distance * safe_distance);
                let clamped_magnitude = force_magnitude.min(2.0);

                let force_direction = if distance > 0.01 {
                    delta.normalize()
                } else {
                    Vec3::Y
                };

                // Rotate arrow to point along the force direction
                let angle = force_direction.y.atan2(force_direction.x);
                scene.set_rotation(vid, Quat::from_rotation_z(angle - PI / 2.0));

                // Scale arrow length based on force magnitude
                let scale = clamped_magnitude * 0.4;
                scene.set_scale(vid, vec3(1.0, scale, 1.0));
            }
        });
    }

    let mut timeline = Timeline::new();
    timeline.wait_until(6.0);
    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}