// src/scene/mod.rs

pub mod drawable_props;
use crate::renderer::mesh::MeshInstance;
pub use drawable_props::{DrawableProps, SharedProps};

use std::collections::HashMap;
use std::sync::Arc;

use glam::{Mat4, Vec3};
use hecs::World;

use crate::{
    camera::Camera,
    // NEW: Sangh/Sync system imports
    core::sync::{BoxedSangh, SyncBoundary, Syncable},
    layout::placement::Place,
    renderer::renderer::{DrawableInstance, DrawableKind},
    tattva::Tattva,
    timeline::Timeline,
};

/// Stable id type for tattvas.
pub type TattvaId = usize;

/// Object-safe trait for something that can be drawn inside a render pass.
pub trait Drawable: Send + Sync {
    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>);
}

pub struct Scene {
    // --- NEW: ECS & Math Sync Logic ---
    pub world: World,
    pub sync_boundary: SyncBoundary,
    pub sanghs: Vec<Box<BoxedSangh>>,

    // --- Authoritative Tattvas & Drawables ---
    pub tattvas: Vec<Box<dyn Tattva>>,
    pub pending_drawable_props: HashMap<TattvaId, DrawableProps>,
    pub drawables: Vec<DrawableInstance>,

    // --- Time & animation ---
    pub scene_time: f32,
    pub play_until: Option<f32>,
    pub timelines: HashMap<String, Timeline>,

    // --- Identity bookkeeping ---
    next_tattva_id: TattvaId,
    tattva_id_map: HashMap<TattvaId, usize>,

    // --- Camera & global transforms ---
    pub camera: Camera,
    pub global_model: Mat4,

    // --- GPU resources ---
    pub texture_cache: HashMap<TattvaId, TattvaResources>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            // Initialize new ECS fields
            world: World::new(),
            sync_boundary: SyncBoundary::new(),
            sanghs: Vec::new(),

            tattvas: Vec::new(),
            pending_drawable_props: HashMap::new(),
            drawables: Vec::new(),
            scene_time: 0.0,
            play_until: None,
            timelines: HashMap::new(),
            next_tattva_id: 1,
            tattva_id_map: HashMap::new(),
            texture_cache: HashMap::new(),
            camera: Camera::default(),
            global_model: Mat4::IDENTITY,
        }
    }

    // ---------------------------------------------------------------------
    // NEW: Sangh (Math Primitive) Management
    // ---------------------------------------------------------------------

    /// High-level helper to add objects.
    /// If it's a Sangh, it adds it to the math/ECS sync list.
    pub fn add<T: Syncable + 'static>(&mut self, sangh: T) {
        self.add_sangh(sangh);
    }

    /// Adds a mathematical Sangh (like Circle, Line) that gets projected into the ECS.
    pub fn add_sangh<T: Syncable + 'static>(&mut self, sangh: T) {
        self.sanghs.push(Box::new(sangh));
    }

    pub fn spawn_sangh<T: Tattva + 'static>(&mut self, tattva: T) -> TattvaId {
        self.spawn(tattva, DrawableProps::default())
    }

    /// Syncs high-level Sanghs to the low-level ECS World.
    /// Should be called every frame in the app loop before rendering.
    pub fn sync(&mut self) {
        for sangh in self.sanghs.iter_mut() {
            sangh.sync_to_world(&mut self.world, &mut self.sync_boundary);
        }
    }

    // ---------------------------------------------------------------------
    // Existing Tattva management
    // ---------------------------------------------------------------------

    fn add_tattva_internal(&mut self, t: Box<dyn Tattva>) -> TattvaId {
        let id = self.next_tattva_id;
        self.next_tattva_id = self.next_tattva_id.wrapping_add(1);

        self.tattvas.push(t);
        self.tattva_id_map.insert(id, self.tattvas.len() - 1);
        id
    }

    pub fn get_tattva(&self, id: TattvaId) -> Option<&Box<dyn Tattva>> {
        self.tattva_id_map
            .get(&id)
            .and_then(|&i| self.tattvas.get(i))
    }

    pub fn tattva_ids(&self) -> impl Iterator<Item = TattvaId> + '_ {
        self.tattva_id_map.keys().copied()
    }

    pub fn override_mesh(&mut self, tattva_id: usize, mesh: Arc<MeshInstance>) {
        for d in self.drawables.iter_mut() {
            if d.tattva_id == Some(tattva_id) {
                d.mesh = Some(mesh.clone());
                return;
            }
        }
    }

    pub fn spawn<T: Tattva + 'static>(&mut self, tattva: T, props: DrawableProps) -> TattvaId {
        let id = self.add_tattva_internal(Box::new(tattva));
        self.drawables.push(DrawableInstance {
            mesh: None,
            props,
            tattva_id: Some(id),
            kind: DrawableKind::Mesh,
        });
        id
    }

    // ---------------------------------------------------------------------
    // Camera & Movement
    // ---------------------------------------------------------------------

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn update(&mut self, dt: f32) {
        self.scene_time += dt;

        // Update Timelines
        let mut timelines = std::mem::take(&mut self.timelines);
        for (_, tl) in timelines.iter_mut() {
            tl.update(self.scene_time, self);
        }
        self.timelines = timelines;

        // IMPORTANT: Sync math primitives to the ECS world after
        // timelines might have moved them
        self.sync();
    }

    pub fn clear(&mut self) {
        self.world = World::new();
        self.sync_boundary = SyncBoundary::new();
        self.sanghs.clear();
        self.tattvas.clear();
        self.drawables.clear();
        self.timelines.clear();
        self.scene_time = 0.0;
        self.play_until = None;
        self.tattva_id_map.clear();
        self.texture_cache.clear();
        self.next_tattva_id = 1;
        self.camera = Camera::default();
        self.global_model = Mat4::IDENTITY;
    }

    pub fn compute_last_scheduled_time(&self) -> Option<f32> {
        let mut max_t: Option<f32> = None;
        for timeline in self.timelines.values() {
            for sa in &timeline.scheduled {
                let end = sa.start_time + sa.duration;
                max_t = Some(max_t.map_or(end, |m| m.max(end)));
            }
        }
        max_t
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TattvaResources {
    pub bind_group: Arc<wgpu::BindGroup>,
}
