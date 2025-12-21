// src/config/preview_config.rs

#[derive(Debug, Clone)]
pub(crate) struct PreviewConfig {
    pub text_px_per_world_unit: f32,
}

impl Default for PreviewConfig {
    fn default() -> Self {
        Self {
            // Fast, interactive default
            text_px_per_world_unit: 128.0,
        }
    }
}
