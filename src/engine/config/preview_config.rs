// src/config/preview_config.rs

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct PreviewConfig {
    #[serde(default = "default_text_px_per_world_unit")]
    pub text_px_per_world_unit: f32,
    #[serde(default = "default_fps")]
    pub fps: u32,
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            // Fast, interactive default
            text_px_per_world_unit: default_text_px_per_world_unit(),
            fps: default_fps(),
        }
    }
}

fn default_text_px_per_world_unit() -> f32 {
    128.0
}

fn default_fps() -> u32 {
    60
}

impl PreviewConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)?;
        let file_cfg: FileConfig = toml::from_str(&contents)?;

        Ok(file_cfg.preview.unwrap_or_default())
    }

    pub fn load_nearest_project_file(start_dir: impl AsRef<Path>) -> Result<Self> {
        let Some(project_dir) = find_nearest_cargo_dir(start_dir.as_ref()) else {
            return Ok(Self::default());
        };

        Self::load(project_dir.join("murali.toml"))
    }
}

#[derive(Debug, Deserialize)]
struct FileConfig {
    pub preview: Option<PreviewConfig>,
}

fn find_nearest_cargo_dir(start_dir: &Path) -> Option<PathBuf> {
    let mut current = Some(start_dir);
    while let Some(dir) = current {
        if dir.join("Cargo.toml").exists() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}
