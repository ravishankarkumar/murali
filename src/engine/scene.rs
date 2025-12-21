//src/engine/scene.rs

pub use crate::frontend::props::{DrawableProps, SharedProps};

use glam::Mat4;
use std::collections::HashMap;
use std::sync::Arc;

use crate::engine::camera::Camera;
use crate::engine::timeline::Timeline;
use crate::frontend::{Tattva, TattvaId, tattva_trait::TattvaTrait};

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

    /// Retrieves a Tattva for inspection or mutation.
    pub fn get_tattva_any_mut(&mut self, id: TattvaId) -> Option<&mut (dyn TattvaTrait + '_)> {
        match self.tattvas.get_mut(&id) {
            Some(b) => Some(b.as_mut()),
            None => None,
        }
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
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
