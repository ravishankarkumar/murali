// src/frontend/collection/mod.rs

pub mod composite;
pub mod primitives;
pub mod text;

pub mod prelude {
    pub use super::composite::*;
    pub use super::primitives::*;
    pub use super::text::*;
}