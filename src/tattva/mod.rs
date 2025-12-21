//! Public exports for the tattva submodule.

pub mod prelude {
    pub use crate::tattva::{Tattva, TattvaExt};
}

// keep per-file modules
pub mod trait_tattva;
pub use trait_tattva::{Tattva, TattvaExt};
pub mod mesh_only_tattva;

pub mod square;
pub use square::Square;

pub mod circle;
pub use circle::Circle;

pub mod line;
pub use line::Line;

pub mod cube;
pub use cube::Cube;

// pub mod axes;
// pub use axes::Axes;


pub mod props;
pub use props::TattvaProps;