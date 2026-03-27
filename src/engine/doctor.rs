use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ToolStatus {
    pub name: &'static str,
    pub path: Option<PathBuf>,
    pub required_for: &'static str,
}

impl ToolStatus {
    pub fn is_available(&self) -> bool {
        self.path.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub latex: ToolStatus,
    pub dvisvgm: ToolStatus,
    pub ffmpeg: ToolStatus,
}

impl DoctorReport {
    pub fn gather() -> Self {
        Self {
            latex: ToolStatus {
                name: "latex",
                path: find_in_path("latex"),
                required_for: "LaTeX compilation",
            },
            dvisvgm: ToolStatus {
                name: "dvisvgm",
                path: find_in_path("dvisvgm"),
                required_for: "LaTeX SVG conversion",
            },
            ffmpeg: ToolStatus {
                name: "ffmpeg",
                path: find_in_path("ffmpeg"),
                required_for: "Video export assembly",
            },
        }
    }

    pub fn render_text(&self) -> String {
        let mut out = String::new();
        out.push_str("Murali Doctor\n");
        out.push_str("=============\n\n");
        for tool in [&self.latex, &self.dvisvgm, &self.ffmpeg] {
            let status = match &tool.path {
                Some(path) => format!("OK ({})", path.display()),
                None => "MISSING".to_string(),
            };
            out.push_str(&format!(
                "- {}: {}\n  needed for: {}\n",
                tool.name, status, tool.required_for
            ));
        }

        if !self.latex.is_available() || !self.dvisvgm.is_available() {
            out.push_str(
                "\nLaTeX note:\n\
                 Murali's current LaTeX path needs both `latex` and `dvisvgm` available on PATH.\n",
            );
        }

        out
    }
}

fn find_in_path(binary: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).find_map(|dir| find_executable(&dir, binary))
}

fn find_executable(dir: &Path, binary: &str) -> Option<PathBuf> {
    let candidate = dir.join(binary);
    if candidate.is_file() {
        return Some(candidate);
    }

    #[cfg(target_os = "windows")]
    {
        let exe = dir.join(format!("{binary}.exe"));
        if exe.is_file() {
            return Some(exe);
        }
    }

    None
}
