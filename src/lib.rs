pub mod backend;
pub mod colors;
pub mod engine;
pub mod frontend;
pub mod math;
pub mod palette;
pub mod positions;
pub mod prelude;
pub mod projection;
pub mod resource;
pub mod utils;

// Re-export common types for ergonomics
pub use engine::app::App;
pub use engine::render::RenderOptions;
pub use engine::scene::{GifCapture, Scene, ScreenshotCapture};
pub use frontend::Tattva;
pub use palette::Palette;
