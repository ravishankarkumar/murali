pub mod prelude;
pub mod backend;
pub mod engine;
pub mod math;
pub mod projection;
pub mod resource;
pub mod frontend;

// Re-export common types for ergonomics
pub use engine::app::App;
pub use engine::scene::Scene;
pub use frontend::Tattva;
pub use engine::render::RenderOptions;