/// Force Field Example with Updaters
/// Demonstrates a moving charged particle affecting a grid of force vectors
/// Each vector updates based on the inverse square law
use glam::{Vec2, Vec3, Vec4, vec3};
use murali::App;
use murali::engine::scene::Scene;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::arrow::Arrow;
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use std::f32::consts::PI;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Electric Force Field", 0.36)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new("Force vectors update based on charged particle position", 0.14)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, 3.2, 0.0),
    );

    // Create the charged particle (positive charge - red)
    let particle_radius = 0.2;
    let particle = Circle::new(particle_radius, 24, Vec4::new(1.0, 0.2, 0.2, 1.0))
        .with_stroke(0.04, Vec4::new(1.0, 0.5, 0.5, 1.0));
    
    let particle_id = scene.add_tattva(particle, Vec3::new(-5.0, 0.0, 0.0));

    // Create a grid of force field vectors
    let grid_spacing = 0.8;
    let grid_x_range = -6..=6;
    let grid_y_range = -3..=2;
    
    let mut vector_ids = Vec::new();
    
    for y in grid_y_range {
        for x in grid_x_range.clone() {
            let pos = vec3(x as f32 * grid_spacing, y as f32 * grid_spacing, 0.0);
            
            // Create an arrow that will represent the force vector
            let arrow = Arrow::with_default_tip(
                Vec2::ZERO,
                Vec2::new(0.0, 0.3),  // Initial direction (will be updated)
                0.03,
                Vec4::new(0.4, 0.7, 1.0, 0.8),
            );
            
            let vector_id = scene.add_tattva(arrow, pos);
            vector_ids.push((vector_id, pos));
        }
    }

    // Animate the charged particle moving in a circle
    let duration = 8.0;
    let radius = 3.0;
    
    // We'll use an updater to move the particle in a circular path
    let start_time = scene.scene_time;
    
    scene.add_updater(particle_id, move |scene, particle_id, _dt| {
        let t = scene.scene_time - start_time;
        let angle = (t / duration) * 2.0 * PI;
        
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        
        scene.set_position(particle_id, Vec2::new(x, y));
    });

    // Add updaters to each force vector to respond to the charged particle
    let charge_strength = 2.0;  // Strength of the charge
    
    for (vector_id, vector_pos) in vector_ids {
        scene.add_updater(vector_id, move |scene, vid, _dt| {
            // Get particle position
            if let Some(particle_tattva) = scene.get_tattva_any(particle_id) {
                let p_props = murali::frontend::props::DrawableProps::read(particle_tattva.props());
                let particle_pos = p_props.position;
                drop(p_props);
                
                // Calculate force vector at this grid point
                // F = k * q / r^2, direction away from positive charge
                let delta = vector_pos - particle_pos;
                let distance = delta.length();
                
                // Avoid division by zero and limit maximum force
                let safe_distance = distance.max(0.5);
                let force_magnitude = charge_strength / (safe_distance * safe_distance);
                
                // Limit maximum force for visualization
                let clamped_magnitude = force_magnitude.min(2.0);
                
                // Calculate force direction (away from positive charge)
                let force_direction = if distance > 0.01 {
                    delta.normalize()
                } else {
                    Vec3::Y  // Default direction if too close
                };
                
                // Calculate rotation angle for the vector
                let angle = force_direction.y.atan2(force_direction.x);
                
                // Update the vector's rotation and scale
                if let Some(vector_tattva) = scene.get_tattva_any_mut(vid) {
                    let mut props = murali::frontend::props::DrawableProps::write(vector_tattva.props());
                    
                    // Rotate the vector to point in the force direction
                    props.rotation = glam::Quat::from_rotation_z(angle - PI / 2.0);
                    
                    // Scale based on force magnitude
                    let scale = clamped_magnitude * 0.4;
                    props.scale = vec3(1.0, scale, 1.0);
                    
                    // Color based on force magnitude (blue to red)
                    // This would require updating the line color, which we can't do easily
                    // So we'll just use scale for now
                }
            }
        });
    }

    App::new()?.with_scene(scene).run_app()
}
