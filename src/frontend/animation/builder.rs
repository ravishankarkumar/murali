use crate::frontend::animation::{Ease, MoveTo}; // Updated paths
use crate::frontend::animation::Animation;
use crate::engine::timeline::Timeline;
use crate::frontend::TattvaId;
use glam::Vec3;

/// The types of transformations the Frontend can request.
#[derive(Debug, Clone)]
pub enum AnimKind {
    MoveTo { to: Vec3 },
    // RotateTo { to: glam::Quat },
    // FadeTo { opacity: f32 },
}

/// The accumulated data for a new animation.
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

/// A Fluent API to schedule animations on the Timeline.
pub struct AnimationBuilder<'a> {
    timeline: &'a mut Timeline,
    spec: AnimationSpec,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(timeline: &'a mut Timeline, target_id: TattvaId) -> Self {
        Self {
            timeline,
            spec: AnimationSpec::new(target_id),
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

    pub fn move_to(mut self, to: Vec3) -> Self {
        self.spec.kind = Some(AnimKind::MoveTo { to });
        self
    }

    /// Converts the spec into a concrete animation and registers it with the engine.
    pub fn spawn(self) {
        let spec = self.spec;
        
        // Match the kind to a concrete Animation implementation
        let anim: Box<dyn Animation> = match spec.kind {
            Some(AnimKind::MoveTo { to }) => {
                Box::new(MoveTo::new(
                    spec.target_id, 
                    to, 
                    spec.duration, 
                    spec.ease
                ))
            }
            None => panic!("Murali Error: AnimationBuilder requires a kind (e.g., .move_to()) before .spawn()"),
        };

        // The timeline now handles the scheduling logic
        self.timeline.add_animation(spec.start_time, spec.duration, anim);
    }
}