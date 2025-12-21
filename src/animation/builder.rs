// src/animation/builder.rs
//! Animation builder DSL (AnimKind, AnimationSpec, AnimationBuilder).
//! Uses stable TattvaId for targets.

use crate::animation::{Ease, MoveTo};
use crate::animation::Animation;
use crate::timeline::Timeline;
use crate::scene::TattvaId;
use glam::Vec3;

/// Kind of animation the builder will produce. Extend with RotateTo, ScaleTo, ColorTo, etc.
#[derive(Debug, Clone)]
pub enum AnimKind {
    MoveTo { to: Vec3 },
    // RotateTo { to: glam::Quat }, // add later
    // ScaleTo { to: glam::Vec3 },
    // ColorTo { to: [f32; 3] },
}

/// The spec we accumulate in the builder before creating a concrete Animation.
#[derive(Debug, Clone)]
pub struct AnimationSpec {
    pub target_id: TattvaId,
    pub start_time: f32,
    pub duration: f32,
    pub ease: Ease,
    pub kind: Option<AnimKind>,
}

impl AnimationSpec {
    pub fn new(target_id: TattvaId) -> Self {
        Self {
            target_id,
            start_time: 0.0,
            duration: 1.0,
            ease: Ease::Linear,
            kind: None,
        }
    }
}

/// Fluent builder for animations.
/// Holds a mutable reference to the Timeline so `spawn()` can schedule directly.
pub struct AnimationBuilder<'a> {
    timeline: &'a mut Timeline,
    spec: AnimationSpec,
}

impl<'a> AnimationBuilder<'a> {
    /// Create a new builder for `target_id`.
    pub fn new(timeline: &'a mut Timeline, target_id: TattvaId) -> Self {
        Self {
            timeline,
            spec: AnimationSpec::new(target_id),
        }
    }

    /// Absolute start time in seconds.
    pub fn at(mut self, t: f32) -> Self {
        self.spec.start_time = t;
        self
    }

    /// Set duration in seconds.
    pub fn for_secs(mut self, d: f32) -> Self {
        self.spec.duration = d;
        self
    }

    /// Set easing curve.
    pub fn ease(mut self, e: Ease) -> Self {
        self.spec.ease = e;
        self
    }

    /// Configure MoveTo kind.
    pub fn move_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(AnimKind::MoveTo { to });
        self
    }

    /// Consume the builder and schedule the concrete boxed Animation on the timeline.
    pub fn spawn(mut self) -> crate::timeline::ScheduledHandle {
        let spec = self.spec.clone();
        let anim: Box<dyn Animation> = match spec.kind {
            Some(AnimKind::MoveTo { to }) => {
                Box::new(MoveTo::new(spec.target_id, spec.start_time, spec.duration, to, spec.ease))
            }
            None => {
                panic!("AnimationBuilder.spawn(): no animation kind specified");
            }
        };

        self.timeline.schedule_at(spec.start_time, spec.duration, anim)
    }
}
