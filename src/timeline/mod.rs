// src/timeline/mod.rs
//! Timeline module — scheduling and runtime logic only.
//! The builder DSL lives in `crate::animation::builder`.

use crate::scene::Scene;
use crate::scene::TattvaId;
use std::fmt;

use crate::animation::camera_animation::{CameraAnimKind, CameraAnimate};
use crate::animation::camera_animation_builder::{CameraAnimationBuilder, CameraAnimationSpec};

/// Runtime state of a scheduled animation.
#[derive(Debug, Clone, PartialEq)]
pub enum AnimState {
    Pending,
    Running { start_time: f32 },
    Done,
}

/// A scheduled animation entry: absolute start_time, duration, boxed animation.
pub struct ScheduledAnimation {
    pub start_time: f32,
    pub duration: f32,
    pub anim: Box<dyn crate::animation::Animation>,
    pub state: AnimState,
}

impl ScheduledAnimation {
    pub fn new(start_time: f32, duration: f32, anim: Box<dyn crate::animation::Animation>) -> Self {
        Self {
            start_time,
            duration,
            anim,
            state: AnimState::Pending,
        }
    }
}

/// Convenience handle returned when scheduling an animation.
#[derive(Debug, Clone)]
pub struct ScheduledHandle {
    pub start_time: f32,
    pub duration: f32,
}

impl fmt::Display for ScheduledHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ScheduledHandle(start={}, dur={})",
            self.start_time, self.duration
        )
    }
}

/// The Timeline holds scheduled animations sorted by start_time.
pub struct Timeline {
    pub scheduled: Vec<ScheduledAnimation>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            scheduled: Vec::new(),
        }
    }

    /// Schedule a boxed animation at the given absolute start_time and duration.
    pub fn schedule_at(
        &mut self,
        start_time: f32,
        duration: f32,
        anim: Box<dyn crate::animation::Animation>,
    ) -> ScheduledHandle {
        let sa = ScheduledAnimation::new(start_time, duration, anim);
        self.scheduled.push(sa);
        self.scheduled
            .sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
        ScheduledHandle {
            start_time,
            duration,
        }
    }

    /// Convenience: start a builder for the given target (TattvaId).
    /// The actual builder lives in crate::animation::builder.
    pub fn animate(
        &mut self,
        target_id: TattvaId,
    ) -> crate::animation::builder::AnimationBuilder<'_> {
        crate::animation::builder::AnimationBuilder::new(self, target_id)
    }

    /// Convenience: start a builder with an explicit absolute start_time.
    pub fn animate_at(
        &mut self,
        target_id: TattvaId,
        start_time: f32,
    ) -> crate::animation::builder::AnimationBuilder<'_> {
        crate::animation::builder::AnimationBuilder::new(self, target_id).at(start_time)
    }



// Replace the update signature & body with this:

    /// Update: start pending animations (if their start_time has arrived) and advance running ones.
    pub fn update(&mut self, scene_time: f32, scene: &mut Scene) {
        // Naive scan — replace with a cursor / windowed scan for performance later.
        for sa in &mut self.scheduled {
            match sa.state {
                AnimState::Pending => {
                    if scene_time >= sa.start_time {
                        sa.state = AnimState::Running { start_time: sa.start_time };
                        sa.anim.on_start(scene);
                    }
                }
                AnimState::Running { start_time } => {
                    // compute elapsed and apply
                    let elapsed = scene_time - start_time;
                    let t_abs = start_time + elapsed.clamp(0.0, sa.duration);
                    sa.anim.apply_at(scene, t_abs);
                    if elapsed >= sa.duration {
                        sa.state = AnimState::Done;
                        sa.anim.on_finish(scene);
                    }
                }
                AnimState::Done => {}
            }
        }
        // pruning Done entries can be done here if desired
    }


    /// Deterministically apply timeline for a given scene_time (seeking).
    pub fn apply_at(&mut self, scene_time: f32, scene: &mut Scene) {
        for sa in &mut self.scheduled {
            if scene_time < sa.start_time {
                sa.state = AnimState::Pending;
            } else {
                let elapsed = (scene_time - sa.start_time).clamp(0.0, sa.duration);
                let t_abs = sa.start_time + elapsed;
                sa.anim.apply_at(scene, t_abs);
                if elapsed >= sa.duration {
                    sa.state = AnimState::Done;
                } else {
                    sa.state = AnimState::Running {
                        start_time: sa.start_time,
                    };
                }
            }
        }
    }

    /// Convenience: start a camera animation builder
    pub fn animate_camera(&mut self) -> CameraAnimationBuilder<'_> {
        CameraAnimationBuilder::new(self)
    }

    /// Convenience: start a camera animation builder with explicit start_time
    pub fn animate_camera_at(&mut self, start_time: f32) -> CameraAnimationBuilder<'_> {
        CameraAnimationBuilder::new(self).at(start_time)
    }
}
