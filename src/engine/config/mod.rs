// src/config/mod.rs

//! Central render configuration resolver.
//!
//! This module owns policy:
//! - preview vs export
//! - defaults
//! - file overrides
//!
//! Other modules must only depend on RenderConfig.

use anyhow::Result;
use std::path::Path;

mod export_config;
mod preview_config;

use export_config::ExportConfig;
use preview_config::PreviewConfig;

// -----------------------------------------------------------------------------
// Public, resolved config
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub text_px_per_world_unit: f32,
}

// -----------------------------------------------------------------------------
// Internal trait (enforces completeness)
// -----------------------------------------------------------------------------

trait ResolveRenderConfig {
    fn resolve(self) -> RenderConfig;
}

impl ResolveRenderConfig for PreviewConfig {
    fn resolve(self) -> RenderConfig {
        RenderConfig {
            text_px_per_world_unit: self.text_px_per_world_unit,
        }
    }
}

impl ResolveRenderConfig for ExportConfig {
    fn resolve(self) -> RenderConfig {
        RenderConfig {
            text_px_per_world_unit: self.text_px_per_world_unit,
        }
    }
}

// -----------------------------------------------------------------------------
// Public constructors
// -----------------------------------------------------------------------------

impl RenderConfig {
    /// Interactive preview configuration (hardcoded defaults).
    pub fn preview() -> Self {
        PreviewConfig::default().resolve()
    }

    /// Export configuration (file-backed, deterministic).
    pub fn export<P: AsRef<Path>>(config_path: Option<P>) -> Result<Self> {
        let export_cfg = match config_path {
            Some(path) => ExportConfig::load(path)?,
            None => ExportConfig::default(),
        };

        Ok(export_cfg.resolve())
    }
}
