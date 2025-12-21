// src/engine/mod.rs

pub mod app;
pub mod scene;
pub mod timeline;
pub mod config;
pub mod camera;

use crate::backend::Backend;
use crate::backend::sync::SyncBoundary;
use crate::engine::scene::Scene;

use winit::window::Window;
use std::sync::Arc;

/// The Engine is the top-level owner of all systems.
pub struct Engine {
    pub scene: Scene,
    pub backend: Backend,
    sync_boundary: SyncBoundary,
}

impl Engine {
    pub async fn new(window: Arc<Window>) -> Self {
        let backend = Backend::new(window).await.expect("Backend init failed");

        Self {
            scene: Scene::new(),
            backend,
            sync_boundary: SyncBoundary::new(),
        }
    }

    /// The Heartbeat: This moves time forward and syncs the layers.
    pub fn update(&mut self, dt: f32) {
        // 1. Advance the Frontend (Animations & Timelines)
        self.scene.update(dt);

        // 2. Perform the Sync Boundary pass
        // Project dirty tattvas and materialize GPU resources
        let device = &self.backend.renderer.device_mgr.device;

        for (_id, tattva) in self.scene.tattvas_iter_mut() {
            self.sync_boundary.sync_tattva(
                &mut self.backend.world,
                device,
                tattva.as_mut(),
            );
        }
    }

    /// Draw the current state of the Backend ECS World.
    pub fn render(&mut self) -> Result<(), anyhow::Error> {
        self.backend
            .renderer
            .render_scene(&self.scene, &self.backend.world)
    }

    pub async fn new_with_scene(
        window: Arc<winit::window::Window>,
        scene: Scene,
    ) -> Self {
        let backend = Backend::new(window).await.expect("Backend creation failed");

        Self {
            scene,
            backend,
            sync_boundary: SyncBoundary::new(),
        }
    }
}
