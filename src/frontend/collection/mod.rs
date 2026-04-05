// src/frontend/collection/mod.rs

pub mod ai;
pub mod composite;
pub mod graph;
pub mod layout;
pub mod math;
pub mod primitives;
pub mod table;
pub mod text;
pub mod utility;

pub mod prelude {
    pub use super::ai::*;
    pub use super::composite::*;
    pub use super::graph::*;
    pub use super::layout::*;
    pub use super::math::*;
    pub use super::primitives::*;
    pub use super::table::*;
    pub use super::text::*;
    pub use super::utility::*;
}
