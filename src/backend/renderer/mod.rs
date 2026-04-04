// src/renderer/mod.rs
//! Renderer module tree — small shim that exposes the real Renderer implementation.

pub mod device;
pub mod mesh;
pub mod renderer;
pub mod vertex; // the large implementation lives here

// Re-export the main types so external code can continue to use `crate::renderer::Renderer` etc.
pub use renderer::Renderer;
pub use renderer::Uniforms;
