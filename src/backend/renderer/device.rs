use anyhow::Result;
use std::sync::Arc;
use wgpu::{Backends, InstanceDescriptor, SurfaceError};
use winit::window::Window;

/// A small wrapper containing the wgpu Device/Queue and the Surface config.
#[derive(Clone)]
pub struct DeviceManager {
    _window: Arc<winit::window::Window>,
    pub surface: Arc<wgpu::Surface<'static>>,
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    // pub config: wgpu::SurfaceConfiguration,
    pub config: std::cell::RefCell<wgpu::SurfaceConfiguration>,
}

impl DeviceManager {
    /// Create a DeviceManager for a given winit window.
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        // Create instance
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = unsafe { instance.create_surface(window.clone()) }?;

        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("No suitable GPU adapters found"))?;

        // Request device & queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("murali-device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        // Surface configuration
        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);

        let format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            _window: window,
            surface: Arc::new(surface),
            device,
            queue,
            // config,
            config: std::cell::RefCell::new(config),
        })
    }

    pub fn max_texture_size(&self) -> u32 {
        self.device.limits().max_texture_dimension_2d
    }

    /// Resize surface on window resize.
    pub fn resize(&self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let mut config = self.config.borrow_mut(); // 🔑 Borrow interiorly
            config.width = new_size.width;
            config.height = new_size.height;
            self.surface.configure(&self.device, &config);
        }
    }

    /// Acquire next surface texture.
    pub fn acquire_frame(
        &self,
    ) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Ok((frame, view))
    }
}

// -----------------------------------------------------------------------------
// Depth texture helper
// -----------------------------------------------------------------------------

pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub format: wgpu::TextureFormat,
}

impl DepthTexture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

    pub fn create(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        let texture = device.create_texture(&desc);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
            format: Self::DEPTH_FORMAT,
        }
    }
}
