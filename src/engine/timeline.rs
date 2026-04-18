use crate::engine::scene::Scene;
use crate::frontend::TattvaId;
use crate::frontend::animation::{
    Animation, Ease, RunSceneCallback, RunSceneCallbackOverTime, builder::AnimationBuilder,
    camera_animation_builder::CameraAnimationBuilder,
};
use crate::frontend::collection::math::equation::VectorEquationHandle;

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
    initialized: bool,     // Track if on_start was called
    reset_performed: bool, // Track if we've performed initial pending reset
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
            reset_performed: false,
        }
    }
}

pub struct Timeline {
    pub scheduled: Vec<ScheduledAnimation>,
    next_order: usize,
    initialized: bool,
    hold_until: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalPlaybackMode {
    Once,
    RoundTrip,
    Loop { repeats: usize },
}

#[derive(Debug, Clone, Copy)]
pub struct SignalPlayback {
    pub start_time: f32,
    pub duration: f32,
    pub ease: Ease,
    pub mode: SignalPlaybackMode,
}

impl SignalPlayback {
    pub fn once(start_time: f32, duration: f32, ease: Ease) -> Self {
        Self {
            start_time,
            duration,
            ease,
            mode: SignalPlaybackMode::Once,
        }
    }

    pub fn round_trip(start_time: f32, duration: f32, ease: Ease) -> Self {
        Self {
            start_time,
            duration,
            ease,
            mode: SignalPlaybackMode::RoundTrip,
        }
    }

    pub fn looped(start_time: f32, duration: f32, repeats: usize, ease: Ease) -> Self {
        Self {
            start_time,
            duration,
            ease,
            mode: SignalPlaybackMode::Loop {
                repeats: repeats.max(1),
            },
        }
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            scheduled: Vec::new(),
            next_order: 0,
            initialized: false,
            hold_until: 0.0,
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
        if !self.initialized {
            // Perform a global reverse reset pass to initialize the scene state at t=0
            for sa in self.scheduled.iter_mut().rev() {
                sa.anim.reset(scene);
                sa.reset_performed = true;
            }
            self.initialized = true;
        }

        for sa in &mut self.scheduled {
            let elapsed = scene_time - sa.start_time;

            if elapsed < 0.0 {
                if !sa.reset_performed {
                    // Note: This chronological pass is still okay for individual resets,
                    // but the first-run block below will handle the global scene initialization.
                    sa.anim.reset(scene);
                    sa.reset_performed = true;
                }
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
        // Perform resets in REVERSE chronological order
        // This ensures the EARLIEST animation for a given Tattva has the final say on the t=0 state.
        for sa in self.scheduled.iter_mut().rev() {
            sa.anim.reset(scene);
            sa.initialized = false;
            sa.reset_performed = true;
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

    pub fn call_at<F>(&mut self, time: f32, callback: F)
    where
        F: FnMut(&mut Scene) + Send + 'static,
    {
        self.add_animation(time, 0.0, Box::new(RunSceneCallback::new(callback)));
    }

    pub fn call_during<F>(&mut self, start_time: f32, duration: f32, callback: F)
    where
        F: FnMut(&mut Scene, f32) + Send + 'static,
    {
        self.add_animation(
            start_time,
            duration.max(0.0),
            Box::new(RunSceneCallbackOverTime::new(callback)),
        );
    }

    pub fn play_signal(&mut self, id: TattvaId, playback: SignalPlayback) {
        match playback.mode {
            SignalPlaybackMode::Once => {
                self.animate(id)
                    .at(playback.start_time)
                    .for_duration(playback.duration)
                    .ease(playback.ease)
                    .propagate()
                    .spawn();
            }
            SignalPlaybackMode::RoundTrip => {
                self.animate(id)
                    .at(playback.start_time)
                    .for_duration(playback.duration)
                    .ease(playback.ease)
                    .propagate()
                    .spawn();

                self.animate(id)
                    .at(playback.start_time + playback.duration)
                    .for_duration(playback.duration)
                    .ease(playback.ease)
                    .propagate_to(0.0)
                    .spawn();
            }
            SignalPlaybackMode::Loop { repeats } => {
                for i in 0..repeats {
                    let cycle_start = playback.start_time + i as f32 * playback.duration;
                    self.animate(id)
                        .at(cycle_start)
                        .for_duration(playback.duration)
                        .ease(playback.ease)
                        .propagate()
                        .spawn();

                    if i + 1 < repeats {
                        self.animate(id)
                            .at(cycle_start + playback.duration)
                            .for_duration(0.0)
                            .ease(playback.ease)
                            .propagate_to(0.0)
                            .spawn();
                    }
                }
            }
        }
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
                .appear()
                .spawn();
        }
    }

    fn stage_targets(scene: &mut Scene, targets: &[TattvaId]) {
        for &id in targets {
            scene.hide_tattva(id);
        }
    }

    pub fn morph_matching_staged(
        &mut self,
        sources: Vec<TattvaId>,
        targets: Vec<TattvaId>,
        scene: &mut Scene,
        start_time: f32,
        duration: f32,
        ease: crate::frontend::animation::Ease,
    ) {
        Self::stage_targets(scene, &targets);
        self.morph_matching(sources, targets, scene, start_time, duration, ease);
    }

    pub fn morph_vector_equations(
        &mut self,
        source: &VectorEquationHandle,
        target: &VectorEquationHandle,
        scene: &mut Scene,
        start_time: f32,
        duration: f32,
        ease: crate::frontend::animation::Ease,
    ) {
        self.morph_matching_staged(
            source.ids().to_vec(),
            target.ids().to_vec(),
            scene,
            start_time,
            duration,
            ease,
        );
    }

    pub fn morph_vector_formulas(
        &mut self,
        source: &VectorEquationHandle,
        target: &VectorEquationHandle,
        scene: &mut Scene,
        start_time: f32,
        duration: f32,
        ease: crate::frontend::animation::Ease,
    ) {
        self.morph_vector_equations(source, target, scene, start_time, duration, ease);
    }

    fn ordered_tattvas(ids: &[TattvaId], scene: &Scene) -> Vec<TattvaId> {
        use crate::frontend::props::DrawableProps;

        let mut ordered = ids.to_vec();
        ordered.sort_by(|a, b| {
            let a_pos = scene
                .get_tattva_any(*a)
                .map(|t| DrawableProps::read(t.props()).position)
                .unwrap_or_default();
            let b_pos = scene
                .get_tattva_any(*b)
                .map(|t| DrawableProps::read(t.props()).position)
                .unwrap_or_default();

            a_pos
                .x
                .partial_cmp(&b_pos.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    b_pos
                        .y
                        .partial_cmp(&a_pos.y)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });
        ordered
    }

    pub fn write_vector_equation(
        &mut self,
        ids: Vec<TattvaId>,
        scene: &Scene,
        start_time: f32,
        duration: f32,
        ease: Ease,
    ) {
        let ordered = Self::ordered_tattvas(&ids, scene);
        if ordered.is_empty() {
            return;
        }

        let window = 1.8_f32;
        let step = duration.max(0.0) / ((ordered.len().saturating_sub(1)) as f32 + window);
        let item_duration = step * window;

        for (idx, id) in ordered.into_iter().enumerate() {
            self.animate(id)
                .at(start_time + idx as f32 * step)
                .for_duration(item_duration)
                .ease(ease)
                .draw()
                .spawn();
        }
    }

    pub fn unwrite_vector_equation(
        &mut self,
        ids: Vec<TattvaId>,
        scene: &Scene,
        start_time: f32,
        duration: f32,
        ease: Ease,
    ) {
        let mut ordered = Self::ordered_tattvas(&ids, scene);
        if ordered.is_empty() {
            return;
        }
        ordered.reverse();

        let window = 1.8_f32;
        let step = duration.max(0.0) / ((ordered.len().saturating_sub(1)) as f32 + window);
        let item_duration = step * window;

        for (idx, id) in ordered.into_iter().enumerate() {
            self.animate(id)
                .at(start_time + idx as f32 * step)
                .for_duration(item_duration)
                .ease(ease)
                .undraw()
                .spawn();
        }
    }

    pub fn end_time(&self) -> f32 {
        let anim_end = self
            .scheduled
            .iter()
            .map(|sa| sa.start_time + sa.duration.max(0.0))
            .fold(0.0, f32::max);
        anim_end.max(self.hold_until)
    }

    /// Ensures the scene runs at least until `timestamp`, even if all animations
    /// finish earlier. Useful for adding a pause at the end of a scene.
    pub fn wait_until(&mut self, timestamp: f32) {
        self.hold_until = self.hold_until.max(timestamp);
    }
}
