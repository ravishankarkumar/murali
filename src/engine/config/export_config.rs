// src/config/export_config.rs

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExportConfig {
    #[serde(default = "default_text_px_per_world_unit")]
    pub text_px_per_world_unit: f32,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            text_px_per_world_unit: default_text_px_per_world_unit(),
        }
    }
}

fn default_text_px_per_world_unit() -> f32 {
    512.0
}

impl ExportConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;
        let file_cfg: FileConfig = toml::from_str(&contents)?;

        Ok(file_cfg.export.unwrap_or_default())
    }
}

#[derive(Debug, Deserialize)]
struct FileConfig {
    pub export: Option<ExportConfig>,
}
