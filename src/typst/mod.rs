//! src/text/mod.rs
//!
//! High-level Typst → SVG → RGBA text pipeline.
//!
//! Invariants:
//! - Text size is defined in WORLD UNITS (not pixels)
//! - `world_height` is REQUIRED
//! - Raster resolution is decoupled from layout
//! - Preview vs export is resolved outside this module

use anyhow::{anyhow, Result};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use twox_hash::XxHash64;

use crate::config::RenderConfig;
use crate::renderer::mesh::Mesh;
use crate::renderer::mesh::typst_quad::make_textured_quad_mesh_for_raster;
use crate::tattva::Tattva;

pub mod cache;
pub mod raster;
pub mod typst_backend;

use cache::{TypstRaster, TypstRasterCache};
use raster::rasterize_svg_to_rgba;
use typst_backend::TypstBackend;

// -----------------------------------------------------------------------------
// Text properties (SEMANTIC, WORLD-SPACE)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TypstProps {
    /// REQUIRED: height of the text bounding box in WORLD UNITS
    pub world_height: f32,

    /// Text color (linear RGBA)
    pub color: [f32; 4],
}

impl TypstProps {
    pub fn new(world_height: f32) -> Self {
        Self {
            world_height,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}

// -----------------------------------------------------------------------------
// TextTattva
// -----------------------------------------------------------------------------

pub struct TypstTattva {
    mesh: Arc<Mesh>,
    pub raster: Arc<TypstRaster>,
    pub color: [f32; 4],
}

impl TypstTattva {
    /// Build a TextTattva from Typst source.
    ///
    /// - Layout is computed in WORLD UNITS
    /// - Raster resolution is controlled by RenderConfig
    pub async fn from_typst_async(
        backend: Arc<TypstBackend>,
        cache: &TypstRasterCache,
        source: &str,
        props: TypstProps,
        render_cfg: &RenderConfig,
    ) -> Result<Self> {
        // ------------------------------------------------------------
        // 1. Typst → SVG
        // ------------------------------------------------------------
        let svg = backend.render_to_svg(source, 0.0)?;

        // ------------------------------------------------------------
        // 2. Intrinsic raster (scale = 1.0, NOT cached)
        //    Used only for layout metrics
        // ------------------------------------------------------------
        let (_rgba, width_px, height_px) = rasterize_svg_to_rgba(&svg, 1.0)?;
        if height_px == 0 {
            return Err(anyhow!("Typst produced zero-height text"));
        }

        let width_px = width_px as f32;
        let height_px = height_px as f32;

        // ------------------------------------------------------------
        // 3. World-space layout
        // ------------------------------------------------------------
        let world_height = props.world_height;
        let world_per_px = world_height / height_px;

        let quad_height_world = world_height;
        let quad_width_world = width_px * world_per_px;

        // ------------------------------------------------------------
        // 4. Raster quality (pixels only)
        // ------------------------------------------------------------
        let px_per_world_unit = render_cfg.text_px_per_world_unit;
        let raster_scale = px_per_world_unit * world_per_px;

        // ------------------------------------------------------------
        // 5. Cached final raster
        // ------------------------------------------------------------
        let cache_key = make_cache_key(&svg, px_per_world_unit);

        let raster = if let Some(r) = cache.get(&cache_key) {
            r
        } else {
            let (rgba, w, h) = rasterize_svg_to_rgba(&svg, raster_scale)?;
            let r = TypstRaster {
                rgba,
                width: w,
                height: h,
                svg: Some(svg.clone()),
            };
            cache.insert(cache_key.clone(), r);
            cache
                .get(&cache_key)
                .expect("Text raster must exist after insertion")
        };

        // ------------------------------------------------------------
        // 6. Build world-space textured quad mesh
        // ------------------------------------------------------------
        let mesh = make_textured_quad_mesh_for_raster(
            quad_width_world,
            quad_height_world,
        );

        Ok(Self {
            mesh,
            raster,
            color: props.color,
        })
    }
}

// -----------------------------------------------------------------------------
// Tattva implementation
// -----------------------------------------------------------------------------

impl Tattva for TypstTattva {
    fn mesh(&self) -> Arc<Mesh> {
        self.mesh.clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// -----------------------------------------------------------------------------
// Cache key helper
// -----------------------------------------------------------------------------

fn make_cache_key(svg: &str, px_per_world_unit: f32) -> String {
    let mut h = XxHash64::with_seed(0);
    svg.hash(&mut h);
    px_per_world_unit.to_bits().hash(&mut h);
    format!("{:016x}", h.finish())
}
