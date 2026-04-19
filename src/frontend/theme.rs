//! Runtime theme support.
//!
//! ## Loading order for [`Theme::load_by_name`]
//! 1. `./themes/{name}.toml` — user's custom project themes directory.
//! 2. Built-in `"dark"` / `"light"` defaults embedded in the binary.
//! 3. Fallback: built-in dark theme.
//!
//! ## Global state
//! A single [`Theme`] is stored in a [`std::sync::OnceLock`] populated at
//! startup by [`Theme::init_global`]. After that, any code can call
//! [`Theme::global`] to read the active theme without any extra plumbing.

use std::{path::Path, sync::OnceLock};

use glam::Vec4;
use serde::Deserialize;

use crate::colors::from_hex;

// ---------------------------------------------------------------------------
// Serde helper: deserialise a Vec4 field from a hex string
// ---------------------------------------------------------------------------

fn deserialize_hex_vec4<'de, D>(deserializer: D) -> Result<Vec4, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    from_hex(&s).map_err(serde::de::Error::custom)
}

// ---------------------------------------------------------------------------
// Theme struct
// ---------------------------------------------------------------------------

/// A complete visual theme used by all Murali mobjects.
#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub name: String,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub background: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub surface: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub surface_alt: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub text_primary: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub text_muted: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub accent: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub accent_alt: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub positive: Vec4,

    #[serde(deserialize_with = "deserialize_hex_vec4")]
    pub warning: Vec4,
}

// ---------------------------------------------------------------------------
// Global singleton
// ---------------------------------------------------------------------------

static GLOBAL_THEME: OnceLock<Theme> = OnceLock::new();

impl Theme {
    /// Populate the global theme. Must be called once at startup (e.g. from
    /// [`crate::engine::config`] during `RenderConfig` construction).
    ///
    /// Subsequent calls are silently ignored — the first write wins.
    pub fn init_global(theme: Theme) {
        let _ = GLOBAL_THEME.set(theme);
    }

    pub fn global() -> &'static Theme {
        GLOBAL_THEME.get_or_init(|| {
            // Try loading from project config
            let cwd = std::env::current_dir().unwrap_or_else(|_| ".".into());
            let project_root = crate::utils::project::find_project_root(&cwd);
            let murali_toml = project_root.join("murali.toml");

            if murali_toml.exists() {
                if let Ok(contents) = std::fs::read_to_string(&murali_toml) {
                    if let Ok(table) = contents.parse::<toml::Table>() {
                        if let Some(theme_name) = table.get("theme").and_then(|v| v.as_str()) {
                            return Theme::load_by_name(theme_name, &project_root);
                        }
                    }
                }
            }

            // Fallback to built-in dark
            toml::from_str(include_str!("../defaults/dark.toml"))
                .expect("built-in dark.toml must be valid")
        })
    }
}

// ---------------------------------------------------------------------------
// Theme discovery & loading
// ---------------------------------------------------------------------------

impl Theme {
    /// Resolve a theme by name, following the lookup chain:
    ///
    /// 1. `{project_root}/themes/{name}.toml` — user-defined theme.
    /// 2. Built-in `"dark"` or `"light"` (aliases accepted).
    /// 3. Built-in dark as final fallback.
    pub fn load_by_name(name: &str, project_root: &Path) -> Self {
        // 1. User project themes directory
        let custom = project_root.join("themes").join(format!("{name}.toml"));

        if custom.exists() {
            match std::fs::read_to_string(&custom) {
                Ok(contents) => match toml::from_str::<Theme>(&contents) {
                    Ok(t) => return t,
                    Err(e) => {
                        eprintln!(
                            "[murali] Warning: failed to parse theme '{name}' at '{}': {e}. Falling back to built-in dark theme.",
                            custom.display()
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "[murali] Warning: failed to read theme '{name}' at '{}': {e}. Falling back to built-in dark theme.",
                        custom.display()
                    );
                }
            }
        }

        // 2. Built-in defaults
        match Self::builtin_named(name) {
            Some(theme) => theme,
            None => {
                eprintln!(
                    "[murali] Warning: unknown theme '{name}'. Falling back to built-in dark theme."
                );
                Self::builtin("dark")
            }
        }
    }

    /// Load a built-in theme by name (`"dark"` or `"light"`).
    /// Falls through to dark on unknown names.
    pub fn builtin(name: &str) -> Self {
        Self::builtin_named(name).unwrap_or_else(|| {
            toml::from_str(include_str!("../defaults/dark.toml"))
                .expect("built-in dark.toml must be valid")
        })
    }

    fn builtin_named(name: &str) -> Option<Self> {
        match name.to_lowercase().replace('_', "-").as_str() {
            "light" | "murali-light" | "classroom-light" => Some(
                toml::from_str(include_str!("../defaults/light.toml"))
                    .expect("built-in light.toml must be valid"),
            ),
            "dark" | "murali-dark" | "ai-under-the-hood" => Some(
                toml::from_str(include_str!("../defaults/dark.toml"))
                    .expect("built-in dark.toml must be valid"),
            ),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Backwards-compatible constructors (kept for existing user code)
// ---------------------------------------------------------------------------

impl Theme {
    /// Dark blue theme used in AI-focused animations.
    ///
    /// Prefer configuring via `murali.toml` (`theme = "dark"`) for new
    /// projects. This constructor is retained for backwards compatibility.
    pub fn ai_under_the_hood() -> Self {
        Self::builtin("dark")
    }

    /// Warm off-white theme for classroom / educational animations.
    ///
    /// Prefer configuring via `murali.toml` (`theme = "light"`) for new
    /// projects. This constructor is retained for backwards compatibility.
    pub fn classroom_light() -> Self {
        Self::builtin("light")
    }
}

#[cfg(test)]
mod tests {
    use super::Theme;
    use std::path::Path;

    #[test]
    fn builtin_named_recognizes_dark_aliases() {
        let theme = Theme::builtin_named("ai-under-the-hood")
            .expect("dark alias should resolve to a built-in theme");
        assert_eq!(theme.name, "murali-dark");
    }

    #[test]
    fn unknown_theme_falls_back_to_dark() {
        let theme = Theme::load_by_name("definitely-not-a-real-theme", Path::new("/tmp"));
        assert_eq!(theme.name, "murali-dark");
    }
}
