use crate::engine::scene::Scene;
use crate::frontend::TattvaId;
use crate::frontend::animation::{
    Animation, builder::AnimationBuilder, camera_animation_builder::CameraAnimationBuilder,
};

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

    pub fn morph_matching(
        &mut self,
        sources: Vec<TattvaId>,
        targets: Vec<TattvaId>,
        scene: &crate::engine::scene::Scene,
        start_time: f32,
        duration: f32,
        ease: crate::frontend::animation::Ease,
    ) {
        use crate::frontend::props::DrawableProps;
        use std::collections::HashMap;

        let mut unmatched_sources = sources.clone();
        let mut unmatched_targets = targets.clone();
        let mut pairs = Vec::new();

        // 1. Match by Tag (Identity)
        let mut source_tags: HashMap<String, Vec<TattvaId>> = HashMap::new();
        for &id in &sources {
            if let Some(t) = scene.get_tattva_any(id) {
                if let Some(tag) = &DrawableProps::read(t.props()).tag {
                    source_tags.entry(tag.clone()).or_default().push(id);
                }
            }
        }

        let mut still_unmatched_targets = Vec::new();
        for &id in &targets {
            let mut matched = false;
            if let Some(t) = scene.get_tattva_any(id) {
                if let Some(tag) = &DrawableProps::read(t.props()).tag {
                    if let Some(ids) = source_tags.get_mut(tag) {
                        if let Some(source_id) = ids.pop() {
                            pairs.push((source_id, id));
                            unmatched_sources.retain(|&x| x != source_id);
                            matched = true;
                        }
                    }
                }
            }
            if !matched {
                still_unmatched_targets.push(id);
            }
        }
        unmatched_targets = still_unmatched_targets;

        // 2. Match by spatial proximity for the remainder
        let mut final_unmatched_targets = Vec::new();
        for &target_id in &unmatched_targets {
            let target_pos = scene
                .get_tattva_any(target_id)
                .map(|t| DrawableProps::read(t.props()).position)
                .unwrap_or_default();

            let mut best_source = None;
            let mut min_dist = f32::MAX;

            for (idx, &source_id) in unmatched_sources.iter().enumerate() {
                let source_pos = scene
                    .get_tattva_any(source_id)
                    .map(|t| DrawableProps::read(t.props()).position)
                    .unwrap_or_default();

                let dist = (target_pos - source_pos).length_squared();
                if dist < min_dist {
                    min_dist = dist;
                    best_source = Some(idx);
                }
            }

            if let Some(idx) = best_source {
                let source_id = unmatched_sources.remove(idx);
                pairs.push((source_id, target_id));
            } else {
                final_unmatched_targets.push(target_id);
            }
        }

        // 3. Bake animations into the timeline
        for (src, tgt) in pairs {
            // Morph geometry
            self.animate(tgt)
                .at(start_time)
                .for_duration(duration)
                .ease(ease)
                .morph_from(src)
                .spawn();

            // Move position to target
            let source_pos = scene
                .get_tattva_any(src)
                .map(|t| DrawableProps::read(t.props()).position)
                .unwrap_or_default();
            let target_pos = scene
                .get_tattva_any(tgt)
                .map(|t| DrawableProps::read(t.props()).position)
                .unwrap_or_default();

            self.animate(tgt)
                .at(start_time)
                .for_duration(duration)
                .ease(ease)
                .move_to(target_pos)
                .from_vec3(source_pos)
                .spawn();

            // Also ensure it fades in if it was hidden
            self.animate(tgt)
                .at(start_time)
                .for_duration(duration * 0.2)
                .fade_to(1.0)
                .from(0.0)
                .spawn();
        }

        // Fade out unmatched sources
        for src in unmatched_sources {
            self.animate(src)
                .at(start_time)
                .for_duration(duration * 0.5)
                .fade_to(0.0)
                .spawn();
        }

        // Fade in unmatched targets (new symbols)
        for tgt in final_unmatched_targets {
            self.animate(tgt)
                .at(start_time + duration * 0.5)
                .for_duration(duration * 0.5)
                .create()
                .spawn();
        }
    }

    pub fn end_time(&self) -> f32 {
        self.scheduled
            .iter()
            .map(|sa| sa.start_time + sa.duration.max(0.0))
            .fold(0.0, f32::max)
    }
}
