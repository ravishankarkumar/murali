// src/config/export_config.rs

use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ExportConfig {
    #[serde(default = "default_text_px_per_world_unit")]
    pub text_px_per_world_unit: f32,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<u32>,
    pub duration_seconds: Option<f32>,
    pub output_dir: Option<PathBuf>,
    pub basename: Option<String>,
    pub video_path: Option<PathBuf>,
    pub gif_path: Option<PathBuf>,
    pub clear_color: Option<[f32; 4]>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            text_px_per_world_unit: default_text_px_per_world_unit(),
            width: None,
            height: None,
            fps: None,
            duration_seconds: None,
            output_dir: None,
            basename: None,
            video_path: None,
            gif_path: None,
            clear_color: None,
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

    pub fn load_nearest_project_file(start_dir: impl AsRef<Path>) -> Result<Self> {
        let Some(project_dir) = find_nearest_cargo_dir(start_dir.as_ref()) else {
            return Ok(Self::default());
        };

        Self::load(project_dir.join("murali.toml"))
    }
}

#[derive(Debug, Deserialize)]
struct FileConfig {
    pub export: Option<ExportConfig>,
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
