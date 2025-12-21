// src/app.rs
use crate::renderer::device::DeviceManager;
use anyhow::Result;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::camera::controller::{
    ActiveCameraController, orbit::OrbitCameraController, pan_zoom::PanZoomCameraController,
};

use crate::label::label_resources::LabelResources;
use crate::renderer::renderer::Renderer;
use crate::scene::Scene;
use crate::config::RenderConfig;

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    scene: Scene,
    camera_controller: ActiveCameraController,
    render_config: RenderConfig,
    label_resources: Option<LabelResources>,
    is_left_mouse_down: bool,
    last_cursor_position: Option<(f64, f64)>,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            window: None,
            renderer: None,
            scene: Scene::default(),
            render_config: RenderConfig::preview(),
            label_resources: None,
            is_left_mouse_down: false,
            last_cursor_position: None,
            camera_controller: ActiveCameraController::Orbit(OrbitCameraController::new(10.0)),
        })
    }

    pub fn with_scene(mut self, scene: Scene) -> Self {
        self.scene = scene;
        self
    }

    pub fn run_app(mut self) -> Result<()> {
        print_camera_help();
        let event_loop = EventLoop::new()?;
        event_loop.run_app(&mut self).map_err(|e| anyhow::anyhow!(e))
    }

    fn materialize_scene(&mut self) {
        let renderer = self.renderer.as_mut().expect("Renderer must exist");
        // Convert high-level Tattvas (like Labels) into Meshes
        crate::draw::materialize_scene(
            &mut self.scene,
            renderer,
            &self.render_config,
            &mut self.label_resources,
        );
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() { return; }

        let window = event_loop.create_window(
            Window::default_attributes()
                .with_title("Murali Renderer")
                .with_inner_size(PhysicalSize::new(1280, 720)),
        ).expect("Failed to create window");

        let arc_window = Arc::new(window);
        let renderer = pollster::block_on(async {
            DeviceManager::new(arc_window.clone()).await.map(Renderer::new)
        }).expect("WGPU Init failed");

        self.renderer = Some(renderer);
        self.window = Some(arc_window);

        // First materialization of labels/static meshes
        self.materialize_scene();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().unwrap();
        let renderer = self.renderer.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                // 1. Sync Math primitives (Circles/Lines) to the ECS World
                self.scene.sync();

                // 2. Render the whole scene (Hybrid: ECS + Meshes)
                if let Err(e) = renderer.render_scene(&self.scene) {
                    if e.to_string().contains("Lost") {
                        renderer.resize(window.inner_size());
                    } else {
                        event_loop.exit();
                    }
                }
                window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                let corrected = enforce_16_9(size);
                if corrected != size { let _ = window.request_inner_size(corrected); }
                renderer.resize(corrected);
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyO) => self.camera_controller = ActiveCameraController::Orbit(OrbitCameraController::new(10.0)),
                    PhysicalKey::Code(KeyCode::KeyP) => self.camera_controller = ActiveCameraController::PanZoom(PanZoomCameraController::new()),
                    PhysicalKey::Code(KeyCode::Escape) => event_loop.exit(),
                    _ => {}
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some((lx, ly)) = self.last_cursor_position {
                    if self.is_left_mouse_down {
                        let delta = glam::vec2((position.x - lx) as f32, (position.y - ly) as f32);
                        self.camera_controller.handle_mouse_drag(delta, self.scene.camera_mut());
                    }
                }
                self.last_cursor_position = Some((position.x, position.y));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left { self.is_left_mouse_down = state == ElementState::Pressed; }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32 * 0.01,
                };
                self.camera_controller.handle_scroll(scroll, self.scene.camera_mut());
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.scene.update(0.016);
    }
}

fn enforce_16_9(size: PhysicalSize<u32>) -> PhysicalSize<u32> {
    let ratio = 16.0 / 9.0;
    let w = size.width as f32;
    let h = size.height as f32;
    if (w / h) > ratio { PhysicalSize::new((h * ratio) as u32, size.height) }
    else { PhysicalSize::new(size.width, (w / ratio) as u32) }
}

fn print_camera_help() {
    println!("\n🎥 Controls: [O] Orbit | [P] PanZoom | [Drag] Move | [Wheel] Zoom\n");
}