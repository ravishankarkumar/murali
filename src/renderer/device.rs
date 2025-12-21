// src/renderer/device.rs

use anyhow::Result;
use std::sync::Arc;
use wgpu::{Backends, InstanceDescriptor, SurfaceConfiguration, SurfaceError, TextureFormat};
use winit::window::Window;

/// A small wrapper containing the wgpu Device/Queue and the Surface config.
pub struct DeviceManager {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: SurfaceConfiguration,
}

impl DeviceManager {
    /// Create a DeviceManager for a given winit window.
    /// This chooses a suitable adapter and creates device & queue.
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        // Create instance
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // Create surface from the winit window
        let surface = unsafe { instance.create_surface(window.clone()) }?;

        // Request an adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("No suitable GPU adapters found"))?;

        // Request device and queue
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

        // Surface configuration
        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);

        // Choose a format
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

        Ok(DeviceManager {
            surface,
            device,
            queue,
            config,
        })
    }

    pub fn max_texture_size(&self) -> u32 {
        self.device.limits().max_texture_dimension_2d
    }

    /// Resize the surface config when the window size changes.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width.max(1);
        self.config.height = height.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    /// Acquire the next surface texture and produce a view for rendering.
    /// Returns (surface_texture, texture_view) or a SurfaceError.
    pub fn acquire_frame(&self) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Ok((frame, view))
    }
}
