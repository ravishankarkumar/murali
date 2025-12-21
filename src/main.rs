use murali::prelude::*;
use murali::sangh::shapes::Circle; // Fix the Circle import
use std::sync::Arc;
use winit::window::Window;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    
    // In winit 0.30, we define attributes first
    let window_attributes = Window::default_attributes()
        .with_title("Murali Engine v2")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0));
    
    // Create the window (using the new 0.30 way)
    let window = Arc::new(event_loop.create_window(window_attributes)?);
    
    let mut scene = Scene::new();
    let renderer = murali::renderer::Renderer::new(window.clone()).await;

    // We need to give every Sangh a unique ID for the SyncBoundary
    let circle = Sangh::new(1, Circle::new(0.5)); 
    scene.add(circle);

    event_loop.run(move |event, elwt| {
        match event {
            winit::event::Event::AboutToWait => {
                scene.update(0.016);
                window.request_redraw();
            }
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::RedrawRequested, .. } => {
                renderer.render(&scene.world);
            }
            winit::event::Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            }
            _ => {}
        }
    })?;

    Ok(())
}