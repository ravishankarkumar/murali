use crate::engine::scene::Scene;
use crate::frontend::TattvaId;
use crate::frontend::animation::{Animation, builder::AnimationBuilder, camera_animation_builder::CameraAnimationBuilder};

#[derive(Debug, Clone, PartialEq)]
pub enum AnimState {
    Pending,
    Running,
    Done,
}

pub struct ScheduledAnimation {
    pub order: usize,
    pub start_time: f32,
    pub duration: f32,
    pub anim: Box<dyn Animation>,
    pub state: AnimState,
    initialized: bool, // Track if on_start was called
}

impl ScheduledAnimation {
    pub fn new(order: usize, start_time: f32, duration: f32, anim: Box<dyn Animation>) -> Self {
        Self {
            order,
            start_time,
            duration: duration.max(0.0),
            anim,
            state: AnimState::Pending,
            initialized: false,
        }
    }
}

pub struct Timeline {
    pub scheduled: Vec<ScheduledAnimation>,
    next_order: usize,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            scheduled: Vec::new(),
            next_order: 0,
        }
    }

    pub fn add_animation(&mut self, start_time: f32, duration: f32, anim: Box<dyn Animation>) {
        let order = self.next_order;
        self.next_order += 1;
        self.scheduled
            .push(ScheduledAnimation::new(order, start_time, duration, anim));
        self.scheduled.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap()
                .then(a.order.cmp(&b.order))
        });
    }

    /// Advances the timeline frame-by-frame.
    pub fn update(&mut self, scene_time: f32, scene: &mut Scene) {
        for sa in &mut self.scheduled {
            let elapsed = scene_time - sa.start_time;

            if elapsed < 0.0 {
                sa.state = AnimState::Pending;
                continue;
            }

            // Trigger initialization if this is the first time we hit this animation
            if !sa.initialized {
                sa.anim.on_start(scene);
                sa.initialized = true;
            }

            if sa.duration <= f32::EPSILON || elapsed >= sa.duration {
                if sa.state != AnimState::Done {
                    sa.anim.apply_at(scene, 1.0); // Ensure it finishes exactly at 1.0
                    sa.anim.on_finish(scene);
                    sa.state = AnimState::Done;
                }
            } else {
                sa.state = AnimState::Running;
                let t = (elapsed / sa.duration).clamp(0.0, 1.0);
                sa.anim.apply_at(scene, t); // Pass normalized 0.0 -> 1.0
            }
        }
    }

    /// Jump to a specific point in time (Seek).
    /// Critical for "Previewing" in an editor.
    pub fn seek_to(&mut self, scene_time: f32, scene: &mut Scene) {
        for sa in &mut self.scheduled {
            sa.anim.reset(scene);
            sa.initialized = false;
            sa.state = AnimState::Pending;
        }
        self.update(scene_time, scene);
    }

    // --- Fluent API Helpers ---

    pub fn animate(&mut self, id: TattvaId) -> AnimationBuilder<'_> {
        AnimationBuilder::new(self, id)
    }

    pub fn animate_camera(&mut self) -> CameraAnimationBuilder<'_> {
        CameraAnimationBuilder::new(self)
    }

    pub fn end_time(&self) -> f32 {
        self.scheduled
            .iter()
            .map(|sa| sa.start_time + sa.duration.max(0.0))
            .fold(0.0, f32::max)
    }
}
