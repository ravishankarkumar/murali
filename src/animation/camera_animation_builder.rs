// src/animation/camera_animation_builder.rs
//! Camera animation builder

use glam::Vec3;

use crate::{
    animation::{camera_animation::{CameraAnimKind, CameraAnimate}, Ease},
    timeline::{ScheduledHandle, Timeline},
};

/// Camera animation spec (owned)
#[derive(Debug, Clone)]
pub struct CameraAnimationSpec {
    pub start_time: f32,
    pub duration: f32,
    pub ease: Ease,
    pub kind: Option<CameraAnimKind>,
}

impl CameraAnimationSpec {
    pub fn new() -> Self {
        Self {
            start_time: 0.0,
            duration: 1.0,
            ease: Ease::Linear,
            kind: None,
        }
    }
}

/// Builder for camera animations.
pub struct CameraAnimationBuilder<'a> {
    timeline: &'a mut Timeline,
    spec: CameraAnimationSpec,
}

impl<'a> CameraAnimationBuilder<'a> {
    pub fn new(timeline: &'a mut Timeline) -> Self {
        Self {
            timeline,
            spec: CameraAnimationSpec::new(),
        }
    }

    pub fn at(mut self, t: f32) -> Self {
        self.spec.start_time = t;
        self
    }

    pub fn for_secs(mut self, d: f32) -> Self {
        self.spec.duration = d;
        self
    }

    pub fn ease(mut self, e: Ease) -> Self {
        self.spec.ease = e;
        self
    }

    /// Move camera position
    pub fn move_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(CameraAnimKind::MoveTo { to });
        self
    }

    /// Change camera look-at target
    pub fn look_at(mut self, target: Vec3) -> Self {
        self.spec.kind = Some(CameraAnimKind::LookAt { target });
        self
    }

    /// Orthographic zoom (1.0 = no change, <1 zoom in, >1 zoom out)
    pub fn zoom_to(mut self, zoom: f32) -> Self {
        self.spec.kind = Some(CameraAnimKind::ZoomTo { zoom });
        self
    }

    /// Perspective FOV animation (radians)
    pub fn fov_to_rad(mut self, fov_y_rad: f32) -> Self {
        self.spec.kind = Some(CameraAnimKind::FovTo { fov_y_rad });
        self
    }

    /// Spawn and schedule the animation
    pub fn spawn(self) -> ScheduledHandle {
        let spec = self.spec;

        let anim: Box<dyn crate::animation::Animation> = match spec.kind {
            Some(CameraAnimKind::MoveTo { to }) => Box::new(
                CameraAnimate::new_move(spec.start_time, spec.duration, to, spec.ease),
            ),

            Some(CameraAnimKind::LookAt { target }) => Box::new(
                CameraAnimate::new_lookat(spec.start_time, spec.duration, target, spec.ease),
            ),

            Some(CameraAnimKind::ZoomTo { zoom }) => Box::new(
                CameraAnimate::new_zoom(spec.start_time, spec.duration, zoom, spec.ease),
            ),

            Some(CameraAnimKind::FovTo { fov_y_rad }) => Box::new(
                CameraAnimate::new_fov(spec.start_time, spec.duration, fov_y_rad, spec.ease),
            ),

            None => panic!("CameraAnimationBuilder.spawn(): no animation kind specified"),
        };

        self.timeline
            .schedule_at(spec.start_time, spec.duration, anim)
    }
}
