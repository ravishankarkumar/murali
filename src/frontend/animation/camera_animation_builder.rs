// src/frontend/animation/camera_animation_builder.rs

use glam::Vec3;
use crate::frontend::animation::{Ease, Animation};
use crate::frontend::animation::camera_animation::{CameraAnimKind, CameraAnimate};
use crate::engine::timeline::Timeline;

/// Data container for the camera's animation lifecycle.
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

/// A Fluent API for manipulating the viewpoint.
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

    pub fn for_duration(mut self, d: f32) -> Self {
        self.spec.duration = d;
        self
    }

    pub fn ease(mut self, e: Ease) -> Self {
        self.spec.ease = e;
        self
    }

    /// Orbit or translate the camera position.
    pub fn move_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(CameraAnimKind::MoveTo { to });
        self
    }

    /// Adjust where the camera is focusing.
    pub fn look_at(mut self, target: Vec3) -> Self {
        self.spec.kind = Some(CameraAnimKind::LookAt { target });
        self
    }

    /// Animate orthographic zoom level.
    pub fn zoom_to(mut self, zoom: f32) -> Self {
        self.spec.kind = Some(CameraAnimKind::ZoomTo { zoom });
        self
    }

    /// Animate perspective Field of View.
    pub fn fov_to(mut self, fov_y_rad: f32) -> Self {
        self.spec.kind = Some(CameraAnimKind::FovTo { fov_y_rad });
        self
    }

    /// Commit the animation to the timeline.
    pub fn spawn(self) {
        let spec = self.spec;

        let anim: Box<dyn Animation> = match spec.kind {
            Some(CameraAnimKind::MoveTo { to }) => {
                Box::new(CameraAnimate::new_move(to, spec.duration, spec.ease))
            }
            Some(CameraAnimKind::LookAt { target }) => {
                Box::new(CameraAnimate::new_lookat(target, spec.duration, spec.ease))
            }
            Some(CameraAnimKind::ZoomTo { zoom }) => {
                Box::new(CameraAnimate::new_zoom(zoom, spec.duration, spec.ease))
            }
            Some(CameraAnimKind::FovTo { fov_y_rad }) => {
                Box::new(CameraAnimate::new_fov(fov_y_rad, spec.duration, spec.ease))
            }
            None => panic!(
                "Murali Error: CameraAnimationBuilder requires a kind (e.g., .look_at()) before .spawn()"
            ),
        };

        self.timeline
            .add_animation(spec.start_time, spec.duration, anim);
    }
}
