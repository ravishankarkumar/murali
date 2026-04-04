pub mod ecs;
pub mod renderer;
pub mod sync;

use crate::backend::renderer::Renderer;
use crate::backend::renderer::device::DeviceManager;

use hecs::World;
use std::sync::Arc;
use winit::window::Window;

/// Backend owns all GPU-side systems and the ECS world.
pub struct Backend {
    pub device_mgr: DeviceManager,
    pub renderer: Renderer,
    pub world: World,
}

impl Backend {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        // Create GPU device / surface
        let device_mgr = DeviceManager::new(window).await?;

        // Create renderer (needs surface format)
        // let surface_format = device_mgr.clone().config.format;

        let device_mgr_arc = Arc::new(device_mgr.clone());
        let renderer = Renderer::new(device_mgr_arc.clone());

        let world = World::new();

        Ok(Self {
            device_mgr,
            renderer,
            world,
        })
    }

    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        let device_mgr = DeviceManager::new_headless(width, height).await?;

        let device_mgr_arc = Arc::new(device_mgr.clone());
        let renderer = Renderer::new(device_mgr_arc.clone());

        let world = World::new();

        Ok(Self {
            device_mgr,
            renderer,
            world,
        })
    }
}
