// src/animation/camera_animation.rs
//! Camera animations (position, target, zoom / fov)

use glam::Vec3;

use crate::{
    animation::{Animation, Ease},
    camera::{Camera, Projection},
    scene::Scene,
};

/// Camera animation kinds.
///
/// Note:
/// - ZoomTo applies to Orthographic projection
/// - FovTo applies to Perspective projection
#[derive(Debug, Clone)]
pub enum CameraAnimKind {
    MoveTo { to: Vec3 },
    LookAt { target: Vec3 },

    /// Orthographic zoom (changes visible width/height uniformly)
    ZoomTo { zoom: f32 },

    /// Perspective field-of-view animation (radians)
    FovTo { fov_y_rad: f32 },
}

/// Animates scene.camera over time.
pub struct CameraAnimate {
    pub start_time: f32,
    pub duration: f32,
    pub kind: CameraAnimKind,
    pub ease: Ease,

    // captured start state
    from_pos: Option<Vec3>,
    from_target: Option<Vec3>,
    from_width: Option<f32>,
    from_height: Option<f32>,
    from_fov: Option<f32>,
}

impl CameraAnimate {
    pub fn new_move(start: f32, dur: f32, to: Vec3, ease: Ease) -> Self {
        Self {
            start_time: start,
            duration: dur,
            kind: CameraAnimKind::MoveTo { to },
            ease,
            from_pos: None,
            from_target: None,
            from_width: None,
            from_height: None,
            from_fov: None,
        }
    }

    pub fn new_lookat(start: f32, dur: f32, target: Vec3, ease: Ease) -> Self {
        Self {
            start_time: start,
            duration: dur,
            kind: CameraAnimKind::LookAt { target },
            ease,
            from_pos: None,
            from_target: None,
            from_width: None,
            from_height: None,
            from_fov: None,
        }
    }

    pub fn new_zoom(start: f32, dur: f32, zoom: f32, ease: Ease) -> Self {
        Self {
            start_time: start,
            duration: dur,
            kind: CameraAnimKind::ZoomTo { zoom },
            ease,
            from_pos: None,
            from_target: None,
            from_width: None,
            from_height: None,
            from_fov: None,
        }
    }

    pub fn new_fov(start: f32, dur: f32, fov_y_rad: f32, ease: Ease) -> Self {
        Self {
            start_time: start,
            duration: dur,
            kind: CameraAnimKind::FovTo { fov_y_rad },
            ease,
            from_pos: None,
            from_target: None,
            from_width: None,
            from_height: None,
            from_fov: None,
        }
    }
}

impl Animation for CameraAnimate {
    fn on_start(&mut self, scene: &mut Scene) {
        let cam = &scene.camera;

        self.from_pos = Some(cam.position);
        self.from_target = Some(cam.target);

        match cam.projection {
            Projection::Orthographic { width, height, .. } => {
                self.from_width = Some(width);
                self.from_height = Some(height);
            }
            Projection::Perspective { fov_y_rad, .. } => {
                self.from_fov = Some(fov_y_rad);
            }
        }
    }

    fn apply_at(&mut self, scene: &mut Scene, t_abs: f32) {
        let elapsed = (t_abs - self.start_time).clamp(0.0, self.duration);
        let t = if self.duration > 0.0 { elapsed / self.duration } else { 1.0 };
        let k = self.ease.eval(t);

        let cam = scene.camera_mut();

        match self.kind {
            CameraAnimKind::MoveTo { to } => {
                let from = self.from_pos.unwrap_or(to);
                cam.position = from.lerp(to, k);
            }

            CameraAnimKind::LookAt { target } => {
                let from = self.from_target.unwrap_or(target);
                cam.target = from.lerp(target, k);
            }

            CameraAnimKind::ZoomTo { zoom } => {
                if let Projection::Orthographic {
                    width,
                    height,
                    near,
                    far,
                } = cam.projection
                {
                    let fw = self.from_width.unwrap_or(width);
                    let fh = self.from_height.unwrap_or(height);

                    cam.projection = Projection::Orthographic {
                        width: fw + (zoom * fw - fw) * k,
                        height: fh + (zoom * fh - fh) * k,
                        near,
                        far,
                    };
                }
            }

            CameraAnimKind::FovTo { fov_y_rad } => {
                if let Projection::Perspective {
                    aspect,
                    near,
                    far,
                    ..
                } = cam.projection
                {
                    let from = self.from_fov.unwrap_or(fov_y_rad);
                    let fov = from + (fov_y_rad - from) * k;

                    cam.projection = Projection::Perspective {
                        fov_y_rad: fov,
                        aspect,
                        near,
                        far,
                    };
                }
            }
        }
    }

    fn on_finish(&mut self, _scene: &mut Scene) {}
}
