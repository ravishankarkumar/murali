/// Force Field with Multiple Charges Example
/// Shows how multiple charged particles create a combined electric field
use glam::{Vec2, Vec3, Vec4, vec3};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::arrow::Arrow;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Electric Field: Multiple Charges", 0.36)
            .with_color(WHITE),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new("Red: Positive Charge | Blue: Negative Charge", 0.14)
            .with_color(GRAY_B),
        Vec3::new(0.0, 3.2, 0.0),
    );

    // Create charged particles
    let particle_radius = 0.18;

    // Positive charge (red) - orbiting
    let positive_charge = Circle::new(particle_radius, 24, RED_C)
        .with_stroke(0.04, RED_B);
    let positive_id = scene.add_tattva(positive_charge, 2.0 * LEFT);

    // Negative charge (blue) - orbiting opposite direction
    let negative_charge = Circle::new(particle_radius, 24, BLUE_D)
        .with_stroke(0.04, BLUE_B);
    let negative_id = scene.add_tattva(negative_charge, 2.0 * RIGHT);

    // Create a grid of force field vectors
    let grid_spacing = 0.7;
    let grid_x_range = -7..=7;
    let grid_y_range = -3..=2;

    let mut vector_ids = Vec::new();

    for y in grid_y_range {
        for x in grid_x_range.clone() {
            let pos = vec3(x as f32 * grid_spacing, y as f32 * grid_spacing, 0.0);

            // Skip positions too close to where charges will be
            let dist_to_pos = (pos - vec3(-2.0, 0.0, 0.0)).length();
            let dist_to_neg = (pos - vec3(2.0, 0.0, 0.0)).length();
            if dist_to_pos < 0.5 || dist_to_neg < 0.5 {
                continue;
            }

            let arrow = Arrow::with_default_tip(
                Vec2::ZERO,
                Vec2::new(0.0, 0.3),
                0.025,
                Vec4::new(BLUE_B.x, BLUE_B.y, BLUE_B.z, 0.7),
            );

            let vector_id = scene.add_tattva(arrow, pos);
            vector_ids.push((vector_id, pos));
        }
    }

    // Animate charges in circular orbits
    let start_time = scene.scene_time;
    let orbit_radius = 2.5;
    let duration = 10.0;

    // Positive charge orbits clockwise
    scene.add_updater(positive_id, move |scene, pid, _dt| {
        let t = scene.scene_time - start_time;
        let angle = (t / duration) * 2.0 * PI;

        let x = orbit_radius * angle.cos();
        let y = orbit_radius * angle.sin();

        scene.set_position_2d(pid, Vec2::new(x, y));
    });

    // Negative charge orbits counter-clockwise
    scene.add_updater(negative_id, move |scene, nid, _dt| {
        let t = scene.scene_time - start_time;
        let angle = -(t / duration) * 2.0 * PI; // Negative for opposite direction

        let x = orbit_radius * angle.cos();
        let y = orbit_radius * angle.sin();

        scene.set_position_2d(nid, Vec2::new(x, y));
    });

    // Add updaters to force vectors - they respond to BOTH charges
    let charge_strength = 1.5;

    for (vector_id, vector_pos) in vector_ids {
        scene.add_updater(vector_id, move |scene, vid, _dt| {
            // Get both particle positions
            let pos_charge_pos = if let Some(p) = scene.get_tattva_any(positive_id) {
                let props = murali::frontend::props::DrawableProps::read(p.props());
                let pos = props.position;
                drop(props);
                Some(pos)
            } else {
                None
            };

            let neg_charge_pos = if let Some(n) = scene.get_tattva_any(negative_id) {
                let props = murali::frontend::props::DrawableProps::read(n.props());
                let pos = props.position;
                drop(props);
                Some(pos)
            } else {
                None
            };

            if let (Some(pos_pos), Some(neg_pos)) = (pos_charge_pos, neg_charge_pos) {
                // Calculate force from positive charge (repulsive - away from charge)
                let delta_pos = vector_pos - pos_pos;
                let dist_pos = delta_pos.length().max(0.3);
                let force_pos_mag = charge_strength / (dist_pos * dist_pos);
                let force_pos = if dist_pos > 0.01 {
                    delta_pos.normalize() * force_pos_mag
                } else {
                    Vec3::ZERO
                };

                // Calculate force from negative charge (attractive - toward charge)
                let delta_neg = neg_pos - vector_pos; // Note: reversed direction
                let dist_neg = delta_neg.length().max(0.3);
                let force_neg_mag = charge_strength / (dist_neg * dist_neg);
                let force_neg = if dist_neg > 0.01 {
                    delta_neg.normalize() * force_neg_mag
                } else {
                    Vec3::ZERO
                };

                // Total force is the vector sum
                let total_force = force_pos + force_neg;
                let force_magnitude = total_force.length().min(2.5);

                if force_magnitude > 0.01 {
                    let force_direction = total_force.normalize();
                    let angle = force_direction.y.atan2(force_direction.x);

                    // Update vector
                    scene.set_rotation(vid, glam::Quat::from_rotation_z(angle - PI / 2.0));
                    scene.set_scale(vid, vec3(1.0, force_magnitude * 0.35, 1.0));
                }
            }
        });
    }

    App::new()?.with_scene(scene).run_app()
}
