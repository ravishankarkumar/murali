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

pub(crate) mod export_config;
mod preview_config;

use export_config::ExportConfig;
use preview_config::PreviewConfig;

// -----------------------------------------------------------------------------
// Public, resolved config
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub text_px_per_world_unit: f32,
    pub fps: u32,
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
            fps: self.fps,
        }
    }
}

impl ResolveRenderConfig for ExportConfig {
    fn resolve(self) -> RenderConfig {
        RenderConfig {
            text_px_per_world_unit: self.text_px_per_world_unit,
            fps: self.fps.unwrap_or(60),
        }
    }
}

// -----------------------------------------------------------------------------
// Public constructors
// -----------------------------------------------------------------------------

impl RenderConfig {
    /// Interactive preview configuration loaded from the nearest project
    /// `murali.toml`, falling back to defaults when absent.
    pub fn preview() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        Ok(PreviewConfig::load_nearest_project_file(cwd)?.resolve())
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
