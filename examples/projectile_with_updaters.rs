/// Projectile Motion with Updaters Example
/// Demonstrates the updater system by showing a ball in projectile motion
/// with velocity vectors and position text that update every frame
use glam::{Vec2, Vec3, Vec4, vec3};
use murali::App;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::primitives::circle::Circle;
use murali::frontend::collection::primitives::line::Line;
use murali::frontend::collection::text::label::Label;
use murali::frontend::collection::utility::TracedPath;
use murali::frontend::layout::Direction;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();

    // Title
    let title_id = scene.add_tattva(
        Label::new("Projectile Motion with Updaters", 0.36)
            .with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    scene.to_edge(title_id, Direction::Up, 0.35);

    // Subtitle
    scene.add_tattva(
        Label::new("Blue: Horizontal Velocity | Green: Vertical Velocity | Yellow: Trajectory", 0.14)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, 3.2, 0.0),
    );

    // Create the projectile (ball)
    let ball_radius = 0.15;
    let ball = Circle::new(ball_radius, 24, Vec4::new(1.0, 0.3, 0.3, 1.0))
        .with_stroke(0.03, Vec4::new(0.8, 0.1, 0.1, 1.0));
    
    let initial_pos = Vec3::new(-5.0, -2.0, 0.0);
    let ball_id = scene.add_tattva(ball, initial_pos);

    // Create velocity vector lines (will be updated by updaters)
    let vx_line = Line::new(
        Vec3::ZERO,
        Vec3::new(1.0, 0.0, 0.0),
        0.08,
        Vec4::new(0.3, 0.7, 1.0, 0.9),
    );
    let vx_line_id = scene.add_tattva(vx_line, initial_pos);

    let vy_line = Line::new(
        Vec3::ZERO,
        Vec3::new(0.0, 1.0, 0.0),
        0.08,
        Vec4::new(0.3, 1.0, 0.3, 0.9),
    );
    let vy_line_id = scene.add_tattva(vy_line, initial_pos);

    // Create position label (will be updated by updater)
    let pos_label = Label::new("", 0.16)
        .with_color(Vec4::new(1.0, 1.0, 1.0, 1.0));
    let pos_label_id = scene.add_tattva(pos_label, Vec3::new(0.0, 3.0, 0.0));

    // Create traced path to show the parabolic trajectory
    let traced_path = TracedPath::new(
        ball_id,
        |ball_pos: Vec3, _ball_rot| ball_pos,
        Vec4::new(1.0, 1.0, 0.3, 0.7),  // Yellow traced path
        0.04,
    )
    .with_min_distance(0.02)
    .with_max_points(1000);
    
    let _traced_path_id = scene.add_tattva(traced_path, Vec3::ZERO);

    // Projectile motion parameters
    let initial_velocity = Vec2::new(3.0, 4.0);  // vx, vy
    let gravity = -5.0;  // acceleration due to gravity
    let duration = 2.0;

    // We'll use an updater to move the ball along a parabolic trajectory
    // instead of using timeline animations
    let start_time = scene.scene_time;
    let start_pos = initial_pos;
    
    scene.add_updater(ball_id, move |scene, ball_id, _dt| {
        let t = scene.scene_time - start_time;
        
        if t <= duration {
            // Calculate position using projectile motion equations
            // x(t) = x0 + vx * t
            // y(t) = y0 + vy * t + 0.5 * g * t^2
            let x = start_pos.x + initial_velocity.x * t;
            let y = start_pos.y + initial_velocity.y * t + 0.5 * gravity * t * t;
            
            scene.set_position(ball_id, Vec2::new(x, y));
        }
    });

    // Add updater for velocity vectors
    let v0 = initial_velocity;
    let g = gravity;
    let t0 = scene.scene_time;
    
    scene.add_updater(vx_line_id, move |scene, line_id, _dt| {
        // Get ball position
        if let Some(ball_tattva) = scene.get_tattva_any(ball_id) {
            let props = murali::frontend::props::DrawableProps::read(ball_tattva.props());
            let ball_pos = props.position;
            drop(props);

            // Calculate current velocity
            let vx = v0.x;  // Constant horizontal velocity
            
            // Update line position and scale
            let line_scale = vx.abs() * 0.5;  // Scale proportional to velocity
            scene.set_position(line_id, Vec2::new(ball_pos.x + ball_radius + 0.1, ball_pos.y));
            
            // Update line scale
            if let Some(line_tattva) = scene.get_tattva_any_mut(line_id) {
                let mut props = murali::frontend::props::DrawableProps::write(line_tattva.props());
                props.scale = vec3(line_scale, 1.0, 1.0);
            }
        }
    });

    scene.add_updater(vy_line_id, move |scene, line_id, _dt| {
        // Get ball position
        if let Some(ball_tattva) = scene.get_tattva_any(ball_id) {
            let props = murali::frontend::props::DrawableProps::read(ball_tattva.props());
            let ball_pos = props.position;
            drop(props);

            // Calculate current velocity
            let t = scene.scene_time - t0;
            let vy = v0.y + g * t;  // Velocity changes with gravity
            
            // Update line position and scale
            let line_scale = vy.abs() * 0.5;  // Scale proportional to velocity
            let line_y_offset = if vy > 0.0 { ball_radius + 0.1 } else { -ball_radius - 0.1 };
            scene.set_position(line_id, Vec2::new(ball_pos.x, ball_pos.y + line_y_offset));
            
            // Update line scale
            if let Some(line_tattva) = scene.get_tattva_any_mut(line_id) {
                let mut props = murali::frontend::props::DrawableProps::write(line_tattva.props());
                let scale_y = if vy > 0.0 { line_scale } else { -line_scale };
                props.scale = vec3(1.0, scale_y, 1.0);
            }
        }
    });

    // Add updater for position label showing velocity
    scene.add_updater(pos_label_id, move |scene, label_id, _dt| {
        // Get ball position
        if let Some(ball_tattva) = scene.get_tattva_any(ball_id) {
            let props = murali::frontend::props::DrawableProps::read(ball_tattva.props());
            let ball_pos = props.position;
            drop(props);

            // Calculate current velocity
            let t = scene.scene_time - start_time;
            let vx = v0.x;
            let vy = v0.y + g * t;

            // Position label above the ball
            scene.set_position(label_id, Vec2::new(ball_pos.x, ball_pos.y + 0.8));
        }
    });

    App::new()?.with_scene(scene).run_app()
}
