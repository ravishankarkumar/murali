use glam::Vec2;

/// Evaluates a quadratic Bézier curve at parameter t [0, 1].
pub fn quadratic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let t_inv = 1.0 - t;
    t_inv * t_inv * p0 + 2.0 * t_inv * t * p1 + t * t * p2
}

/// Evaluates a cubic Bézier curve at parameter t [0, 1].
pub fn cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let t_inv = 1.0 - t;
    let t_inv2 = t_inv * t_inv;
    let t_inv3 = t_inv2 * t_inv;
    let t2 = t * t;
    let t3 = t2 * t;

    t_inv3 * p0 + 3.0 * t_inv2 * t * p1 + 3.0 * t_inv * t2 * p2 + t3 * p3
}
