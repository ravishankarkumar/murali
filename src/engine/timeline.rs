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
    pub start_time: f32,
    pub duration: f32,
    pub anim: Box<dyn Animation>,
    pub state: AnimState,
    initialized: bool, // Track if on_start was called
}

impl ScheduledAnimation {
    pub fn new(start_time: f32, duration: f32, anim: Box<dyn Animation>) -> Self {
        Self {
            start_time,
            duration,
            anim,
            state: AnimState::Pending,
            initialized: false,
        }
    }
}

pub struct Timeline {
    pub scheduled: Vec<ScheduledAnimation>,
}

impl Timeline {
    pub fn new() -> Self {
        Self { scheduled: Vec::new() }
    }

    pub fn add_animation(&mut self, start_time: f32, duration: f32, anim: Box<dyn Animation>) {
        self.scheduled.push(ScheduledAnimation::new(start_time, duration, anim));
        // Keep timeline sorted for efficient scanning
        self.scheduled.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
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

            if elapsed >= sa.duration {
                if sa.state != AnimState::Done {
                    sa.anim.apply_at(scene, 1.0); // Ensure it finishes exactly at 1.0
                    sa.anim.on_finish(scene);
                    sa.state = AnimState::Done;
                }
            } else {
                sa.state = AnimState::Running;
                let t = elapsed / sa.duration; 
                sa.anim.apply_at(scene, t); // Pass normalized 0.0 -> 1.0
            }
        }
    }

    /// Jump to a specific point in time (Seek).
    /// Critical for "Previewing" in an editor.
    pub fn seek_to(&mut self, scene_time: f32, scene: &mut Scene) {
        // Reset initialization for a clean re-run of the state
        for sa in &mut self.scheduled {
            sa.initialized = false;
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
}