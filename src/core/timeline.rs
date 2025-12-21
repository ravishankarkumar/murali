use std::collections::VecDeque;

/// A simple easing enum to control interpolation curves.
#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    SineInOut,
    QuadOut,
}

pub struct ScheduledAnimation {
    pub start_time: f32,
    pub duration: f32,
    pub easing: Easing,
    // This closure mutates the Sangh's semantic state
    pub apply: Box<dyn Fn(f32) + Send + Sync>, 
}

pub struct Timeline {
    current_time: f32,
    animations: VecDeque<ScheduledAnimation>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            animations: VecDeque::new(),
        }
    }

    pub fn add_animation(&mut self, anim: ScheduledAnimation) {
        self.animations.push_back(anim);
    }

    pub fn tick(&mut self, dt: f32) {
        self.current_time += dt;
        
        for anim in self.animations.iter() {
            if self.current_time >= anim.start_time && self.current_time <= anim.start_time + anim.duration {
                let t = (self.current_time - anim.start_time) / anim.duration;
                let alpha = match anim.easing {
                    Easing::Linear => t,
                    Easing::SineInOut => 0.5 * (1.0 - (t * std::f32::consts::PI).cos()),
                    Easing::QuadOut => 1.0 - (1.0 - t) * (1.0 - t),
                };
                (anim.apply)(alpha);
            }
        }
    }

    pub fn time(&self) -> f32 {
        self.current_time
    }
}