// src/latex/tattva.rs
//
// Semantic LaTeX tattva.
// ---------------------
// This type is GPU-agnostic.
// It represents *what* LaTeX exists in the scene, not *how* it is rendered.
//
// GPU resources (raster, mesh, texture) are created later during
// App::materialize_scene(), when renderer + device limits are available.

use std::any::Any;
use std::path::PathBuf;
use std::sync::Arc;

use crate::renderer::mesh::Mesh;
use crate::tattva::Tattva;

use crate::latex::backend::compile_latex_to_svg;
use crate::latex::error::LatexError;

/// Semantic LaTeX object placed in the scene.
///
/// Contains:
/// - original LaTeX source (for debugging / caching)
/// - compiled SVG path (vector, resolution-independent)
/// - desired world-space height
///
/// Does NOT contain:
/// - raster
/// - mesh
/// - GPU texture
pub struct LatexTattva {
    /// Original LaTeX source (math-mode only)
    pub latex_src: String,

    /// Path to compiled SVG (vector form)
    pub svg_path: PathBuf,

    /// Desired height in world units
    pub world_height: f32,
}

impl LatexTattva {
    /// Create a LaTeX tattva.
    ///
    /// This function is intentionally GPU-agnostic.
    /// It only compiles LaTeX → SVG and records layout intent.
    pub fn from_latex(
        latex_src: &str,
        world_height: f32,
    ) -> Result<Self, LatexError> {
        // Compile LaTeX → SVG
        let work_dir = std::env::temp_dir().join("murali_latex");
        let svg = compile_latex_to_svg(latex_src, &work_dir)?;

        Ok(Self {
            latex_src: latex_src.to_string(),
            svg_path: svg.svg_path,
            world_height,
        })
    }
}

impl Tattva for LatexTattva {
    /// Mesh is NOT available at this stage.
    ///
    /// GPU meshes are created during scene materialization.
    fn mesh(&self) -> Arc<Mesh> {
        panic!(
            "LatexTattva mesh requested before materialization. \
             LaTeX GPU resources are created during App::materialize_scene()."
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
