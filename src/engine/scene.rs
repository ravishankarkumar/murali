//src/engine/scene.rs

pub use crate::frontend::props::{DrawableProps, SharedProps};

use glam::{Mat4, Vec2, Vec3, vec2, vec3};
use std::collections::HashMap;

use crate::engine::camera::Camera;
use crate::engine::timeline::Timeline;
use crate::frontend::layout::{Anchor, Bounds, Direction, anchor_for_direction, opposite_anchor};
use crate::frontend::{DirtyFlags, IntoTattva, TattvaId, tattva_trait::TattvaTrait};

/// The Scene represents the authoritative Frontend state.
pub struct Scene {
    /// Authoritative Tattvas (Source of Truth)
    /// We use TattvaTrait to allow different types (Circle, Latex, etc.) in one list.
    pub tattvas: HashMap<TattvaId, Box<dyn TattvaTrait>>,

    /// Time & Animation
    pub scene_time: f32,
    pub timelines: HashMap<String, Timeline>,

    /// Global State
    pub camera: Camera,
    pub global_model: Mat4,

    /// Identity bookkeeping
    next_tattva_id: TattvaId,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            tattvas: HashMap::new(),
            scene_time: 0.0,
            timelines: HashMap::new(),
            camera: Camera::default(),
            global_model: Mat4::IDENTITY,
            next_tattva_id: 1,
        }
    }

    /// Adds a Tattva to the scene and returns its stable ID.
    pub fn add<T: TattvaTrait + 'static>(&mut self, mut tattva: T) -> TattvaId {
        let id = self.next_tattva_id;
        self.next_tattva_id += 1;

        // Tattvas need to know their own ID for animation targeting
        // (Assuming a .set_id() helper on the trait)
        tattva.set_id(id);

        self.tattvas.insert(id, Box::new(tattva));
        id
    }

    /// Adds a concrete state object to the scene at a given position.
    pub fn add_tattva<T>(&mut self, state: T, position: Vec3) -> TattvaId
    where
        T: crate::projection::Project + crate::frontend::layout::Bounded + Send + Sync + 'static,
    {
        let tattva = state.into_tattva();
        {
            let mut props = DrawableProps::write(&tattva.props);
            props.position = position;
        }
        self.add(tattva)
    }

    /// Retrieves a Tattva for inspection or mutation.
    pub fn get_tattva_any_mut(&mut self, id: TattvaId) -> Option<&mut (dyn TattvaTrait + '_)> {
        match self.tattvas.get_mut(&id) {
            Some(b) => Some(b.as_mut()),
            None => None,
        }
    }

    pub fn get_tattva_any(&self, id: TattvaId) -> Option<&(dyn TattvaTrait + '_)> {
        match self.tattvas.get(&id) {
            Some(b) => Some(b.as_ref()),
            None => None,
        }
    }

    pub fn get_tattva_typed<T: 'static>(
        &self,
        id: TattvaId,
    ) -> Option<&crate::frontend::Tattva<T>> {
        self.get_tattva_any(id)?
            .as_any()
            .downcast_ref::<crate::frontend::Tattva<T>>()
    }

    pub fn get_tattva_typed_mut<T: 'static>(
        &mut self,
        id: TattvaId,
    ) -> Option<&mut crate::frontend::Tattva<T>> {
        self.get_tattva_any_mut(id)?
            .as_any_mut()
            .downcast_mut::<crate::frontend::Tattva<T>>()
    }

    /// Primary update loop for the frontend.
    pub fn update(&mut self, dt: f32) {
        self.scene_time += dt;

        // 1. Tick all timelines (this mutates Tattva props/state)
        // We temporarily move them out to avoid borrow checker conflicts with 'self'
        let mut timelines = std::mem::take(&mut self.timelines);
        for (_, tl) in timelines.iter_mut() {
            tl.update(self.scene_time, self);
        }
        self.timelines = timelines;
    }

    /// Returns an iterator over all Tattvas for the Sync Boundary to process.
    pub fn tattvas_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&TattvaId, &mut Box<dyn TattvaTrait>)> {
        self.tattvas.iter_mut()
    }

    pub fn local_bounds(&self, id: TattvaId) -> Option<Bounds> {
        self.get_tattva_any(id).map(|t| t.local_bounds())
    }

    pub fn world_bounds(&self, id: TattvaId) -> Option<Bounds> {
        let tattva = self.get_tattva_any(id)?;
        let local = tattva.local_bounds();
        let props = DrawableProps::read(tattva.props());
        let scaled = local.scale(vec2(props.scale.x, props.scale.y));
        Some(scaled.translate(vec2(props.position.x, props.position.y)))
    }

    pub fn anchor_position(&self, id: TattvaId, anchor: Anchor) -> Option<Vec2> {
        self.world_bounds(id).map(|b| b.anchor(anchor))
    }

    pub fn set_position(&mut self, id: TattvaId, position: Vec2) {
        if let Some(tattva) = self.get_tattva_any_mut(id) {
            let mut props = DrawableProps::write(tattva.props());
            props.position = vec3(position.x, position.y, props.position.z);
            drop(props);
            tattva.mark_dirty(DirtyFlags::TRANSFORM);
        }
    }

    pub fn align_to(&mut self, moving: TattvaId, target: TattvaId, anchor: Anchor) {
        let Some(moving_anchor) = self.anchor_position(moving, anchor) else {
            return;
        };
        let Some(target_anchor) = self.anchor_position(target, anchor) else {
            return;
        };
        let delta = target_anchor - moving_anchor;
        if let Some(bounds) = self.world_bounds(moving) {
            let current_pos = bounds.center();
            let new_pos = match anchor {
                Anchor::Up | Anchor::Down => glam::vec2(current_pos.x, current_pos.y + delta.y),
                Anchor::Left | Anchor::Right => glam::vec2(current_pos.x + delta.x, current_pos.y),
                _ => current_pos + delta,
            };
            self.set_position(moving, new_pos);
        }
    }

    pub fn next_to(
        &mut self,
        moving: TattvaId,
        target: TattvaId,
        direction: Direction,
        padding: f32,
    ) {
        let Some(target_bounds) = self.world_bounds(target) else {
            return;
        };
        let Some(local_bounds) = self.local_bounds(moving) else {
            return;
        };

        let moving_anchor = opposite_anchor(direction);
        let target_anchor = anchor_for_direction(direction);
        let target_point = target_bounds.anchor(target_anchor);
        let local_anchor = local_bounds.anchor(moving_anchor);

        let offset = match direction {
            Direction::Up => vec2(0.0, padding),
            Direction::Down => vec2(0.0, -padding),
            Direction::Left => vec2(-padding, 0.0),
            Direction::Right => vec2(padding, 0.0),
        };

        self.set_position(moving, target_point + offset - local_anchor);
    }

    pub fn to_edge(&mut self, id: TattvaId, direction: Direction, margin: f32) {
        let Some(local_bounds) = self.local_bounds(id) else {
            return;
        };
        let frame = self.frame_bounds();
        let target_anchor = anchor_for_direction(direction);
        let moving_anchor = opposite_anchor(direction);
        let edge_point = frame.anchor(target_anchor);
        let margin_offset = match direction {
            Direction::Up => vec2(0.0, -margin),
            Direction::Down => vec2(0.0, margin),
            Direction::Left => vec2(margin, 0.0),
            Direction::Right => vec2(-margin, 0.0),
        };
        self.set_position(
            id,
            edge_point + margin_offset - local_bounds.anchor(moving_anchor),
        );
    }

    pub fn frame_bounds(&self) -> Bounds {
        let half_height = 4.0;
        let half_width = half_height * (16.0 / 9.0);
        Bounds::new(
            vec2(-half_width, -half_height),
            vec2(half_width, half_height),
        )
    }

    pub fn clear(&mut self) {
        self.tattvas.clear();
        self.timelines.clear();
        self.scene_time = 0.0;
        self.next_tattva_id = 1;
        self.camera = Camera::default();
    }

    // camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Replaces the implementation of an existing Tattva.
    /// This is used for shape morphing where we swap types (e.g., Circle -> Path).
    pub fn replace_tattva(&mut self, id: TattvaId, tattva: Box<dyn TattvaTrait>) {
        self.tattvas.insert(id, tattva);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
