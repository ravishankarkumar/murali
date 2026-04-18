use std::collections::HashMap;
use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use glam::Vec4;
use image::ColorType;
use image::ImageEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};

use crate::engine::Engine;
use crate::engine::config::export_config::ExportConfig;
use crate::engine::render::RenderOptions;
use crate::engine::scene::Scene;
use crate::frontend::theme::Theme;

#[derive(Debug, Clone)]
pub struct ExportSettings {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_seconds: f32,
    pub output_dir: PathBuf,
    pub basename: String,
    pub video_path: Option<PathBuf>,
    pub gif_path: Option<PathBuf>,
    pub capture_gif_dir: Option<PathBuf>,
    pub clear_color: Vec4,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 60,
            duration_seconds: 1.0,
            output_dir: PathBuf::from("renders/frames"),
            basename: "frame".to_string(),
            video_path: Some(PathBuf::from("renders/output.mp4")),
            gif_path: None,
            capture_gif_dir: None,
            clear_color: Vec4::new(0.05, 0.10, 0.15, 1.0),
        }
    }
}

impl ExportSettings {
    pub fn from_scene(scene: &Scene) -> Self {
        let mut settings = Self::default();
        settings.duration_seconds = infer_duration(scene);

        let bg = Theme::global().background;
        settings.clear_color = bg;

        settings
    }

    pub fn from_project_config(scene: &Scene, options: &RenderOptions) -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let cfg = ExportConfig::load_nearest_project_file(cwd)?;

        let mut settings = Self::from_scene(scene);
        settings.width = options
            .resolution
            .map(|(w, _)| w)
            .or(cfg.width)
            .unwrap_or(settings.width);
        settings.height = options
            .resolution
            .map(|(_, h)| h)
            .or(cfg.height)
            .unwrap_or(settings.height);
        settings.fps = options.fps.or(cfg.fps).unwrap_or(settings.fps);
        settings.duration_seconds = cfg.duration_seconds.unwrap_or(settings.duration_seconds);
        if let Some(output_dir) = cfg.output_dir {
            settings.output_dir = output_dir;
        }
        if let Some(basename) = cfg.basename {
            settings.basename = basename;
        }
        if let Some(clear_color) = cfg.clear_color {
            settings.clear_color = Vec4::new(
                clear_color[0],
                clear_color[1],
                clear_color[2],
                clear_color[3],
            );
        }

        settings.video_path = if options.video_enabled() {
            resolve_video_output_path(
                options
                    .output
                    .as_ref()
                    .map(PathBuf::from)
                    .or(cfg.video_path),
            )
        } else {
            None
        };

        settings.gif_path = cfg.gif_path;
        if !options.frames_enabled() {
            settings.output_dir = PathBuf::from("renders/frames");
        }

        Ok(settings)
    }

    pub fn total_frames(&self) -> u32 {
        ((self.duration_seconds.max(0.0) * self.fps.max(1) as f32).round() as u32).saturating_add(1)
    }

    pub fn frame_dt(&self) -> f32 {
        1.0 / self.fps.max(1) as f32
    }

    pub fn frame_path(&self, index: u32) -> PathBuf {
        self.output_dir
            .join(format!("{}_{index:05}.png", self.basename))
    }
}

fn resolve_video_output_path(path: Option<PathBuf>) -> Option<PathBuf> {
    let path = path.unwrap_or_else(|| PathBuf::from("renders/output.mp4"));
    if looks_like_directory_path(&path) {
        let stem = infer_default_export_stem();
        return Some(path.join(format!("{stem}.mp4")));
    }

    Some(path)
}

fn looks_like_directory_path(path: &Path) -> bool {
    if path.as_os_str().is_empty() {
        return true;
    }
    if path.exists() {
        return path.is_dir();
    }
    path.extension().is_none()
}

fn infer_default_export_stem() -> String {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(stem) = exe_path.file_stem().and_then(|s| s.to_str()) {
            if !stem.is_empty() {
                return sanitize_stem(stem);
            }
        }
    }

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("murali_export_{ts}")
}

fn sanitize_stem(stem: &str) -> String {
    let sanitized: String = stem
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();

    if sanitized.is_empty() {
        "murali_output".to_string()
    } else {
        sanitized
    }
}

pub fn infer_duration(scene: &Scene) -> f32 {
    let max_timeline = scene
        .timelines
        .values()
        .map(|timeline| timeline.end_time())
        .fold(0.0, f32::max);
    max_timeline.max(0.1)
}

pub fn export_scene(scene: Scene, settings: &ExportSettings) -> Result<()> {
    fs::create_dir_all(&settings.output_dir)?;
    clear_existing_frame_outputs(settings)?;
    let mut marker_gif_state = MarkerGifState::from_settings(settings, &scene)?;

    let mut engine = pollster::block_on(Engine::new_headless_with_scene(
        scene,
        settings.width,
        settings.height,
    ))?;
    engine
        .backend
        .renderer
        .resize(winit::dpi::PhysicalSize::new(
            settings.width,
            settings.height,
        ));
    engine.backend.renderer.clear_color = wgpu::Color {
        r: settings.clear_color.x as f64,
        g: settings.clear_color.y as f64,
        b: settings.clear_color.z as f64,
        a: settings.clear_color.w as f64,
    };

    for next_frame in 0..settings.total_frames() {
        let dt = if next_frame == 0 {
            0.0
        } else {
            settings.frame_dt()
        };

        let frame_start = Instant::now();
        if next_frame == 0 {
            eprintln!("Export frame 1: starting update");
        }
        engine.update(dt);
        if next_frame == 0 {
            eprintln!(
                "Export frame 1: update finished in {:.2?}",
                frame_start.elapsed()
            );
        }

        let render_start = Instant::now();
        let image = engine
            .backend
            .renderer
            .render_to_image(&engine.scene, &engine.backend.world)
            .with_context(|| format!("Failed to render export frame {}", next_frame))?;
        if next_frame == 0 {
            eprintln!(
                "Export frame 1: render/readback finished in {:.2?}",
                render_start.elapsed()
            );
        }

        save_requested_screenshots(&image, &engine.scene, settings, &mut marker_gif_state)
            .with_context(|| {
                format!(
                    "Failed to save requested screenshot at frame {}",
                    next_frame
                )
            })?;

        let save_start = Instant::now();
        save_png_fast(&image, &settings.frame_path(next_frame))
            .with_context(|| format!("Failed to save export frame {}", next_frame))?;
        if next_frame == 0 {
            eprintln!(
                "Export frame 1: png save finished in {:.2?}",
                save_start.elapsed()
            );
            eprintln!(
                "Export frame 1: total frame time {:.2?}",
                frame_start.elapsed()
            );
        }

        if next_frame == 0 || next_frame + 1 == settings.total_frames() || next_frame % 10 == 0 {
            eprintln!(
                "Export progress: frame {}/{}",
                next_frame + 1,
                settings.total_frames()
            );
        }
    }

    if let Some(video_path) = &settings.video_path {
        assemble_video(settings, video_path)?;
    }

    if let Some(gif_path) = &settings.gif_path {
        assemble_gif(settings, gif_path)?;
    }

    marker_gif_state.assemble_all(settings)?;

    Ok(())
}

fn assemble_video(settings: &ExportSettings, video_path: &Path) -> Result<()> {
    if !ffmpeg_available() {
        return Err(anyhow::anyhow!(
            "ffmpeg not found in PATH; PNG frames were exported to {}",
            settings.output_dir.display()
        ));
    }

    if let Some(parent) = video_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let pattern = settings
        .output_dir
        .join(format!("{}_%05d.png", settings.basename));

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg(settings.fps.to_string())
        .arg("-i")
        .arg(pattern)
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(video_path)
        .status()
        .context("Failed to spawn ffmpeg for video assembly")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "ffmpeg exited with status {status}; frames remain in {}",
            settings.output_dir.display()
        ));
    }

    Ok(())
}

fn assemble_gif(settings: &ExportSettings, gif_path: &Path) -> Result<()> {
    if !ffmpeg_available() {
        return Err(anyhow::anyhow!(
            "ffmpeg not found in PATH; PNG frames were exported to {}",
            settings.output_dir.display()
        ));
    }

    if let Some(parent) = gif_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let pattern = settings
        .output_dir
        .join(format!("{}_%05d.png", settings.basename));

    // High quality GIF generation using palettegen/paletteuse
    let filter = format!(
        "fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        settings.fps, settings.width
    );

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(pattern)
        .arg("-vf")
        .arg(filter)
        .arg(gif_path)
        .status()
        .context("Failed to spawn ffmpeg for GIF assembly")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "ffmpeg exited with status {status}; frames remain in {}",
            settings.output_dir.display()
        ));
    }

    Ok(())
}

fn ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn clear_existing_frame_outputs(settings: &ExportSettings) -> Result<()> {
    if settings.output_dir.exists() {
        for entry in fs::read_dir(&settings.output_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.starts_with(&format!("{}_", settings.basename)) && name.ends_with(".png") {
                fs::remove_file(path)?;
            }
        }
    }

    Ok(())
}

fn save_requested_screenshots(
    image: &image::RgbaImage,
    scene: &Scene,
    settings: &ExportSettings,
    marker_gif_state: &mut MarkerGifState,
) -> Result<()> {
    marker_gif_state.capture_scene_specs(image, scene, settings)?;

    Ok(())
}

#[derive(Debug, Default)]
struct MarkerGifState {
    output_dir: Option<PathBuf>,
    frame_counts: HashMap<String, usize>,
    screenshot_count: usize,
    scene_screenshot_indices: Vec<usize>,
    scene_gif_indices: Vec<usize>,
}

impl MarkerGifState {
    fn from_settings(settings: &ExportSettings, scene: &Scene) -> Result<Self> {
        for capture in &scene.screenshot_captures {
            if let Some(names) = &capture.names {
                if names.len() != capture.times.len() {
                    return Err(anyhow::anyhow!(
                        "ScreenshotCapture names length ({}) must match times length ({})",
                        names.len(),
                        capture.times.len()
                    ));
                }
            }
        }

        let Some(output_dir) = settings.capture_gif_dir.clone() else {
            return Ok(Self {
                output_dir: None,
                frame_counts: HashMap::new(),
                screenshot_count: 0,
                scene_screenshot_indices: vec![0; scene.screenshot_captures.len()],
                scene_gif_indices: vec![0; scene.gif_captures.len()],
            });
        };
        fs::create_dir_all(&output_dir)?;
        let frames_root = marker_gif_frames_root(&output_dir);
        if frames_root.exists() {
            fs::remove_dir_all(&frames_root)?;
        }
        fs::create_dir_all(&frames_root)?;
        Ok(Self {
            output_dir: Some(output_dir),
            frame_counts: HashMap::new(),
            screenshot_count: 0,
            scene_screenshot_indices: vec![0; scene.screenshot_captures.len()],
            scene_gif_indices: vec![0; scene.gif_captures.len()],
        })
    }

    fn capture_scene_specs(
        &mut self,
        image: &image::RgbaImage,
        scene: &Scene,
        settings: &ExportSettings,
    ) -> Result<()> {
        let current_time = scene.scene_time;

        for (capture_idx, capture) in scene.screenshot_captures.iter().enumerate() {
            while self.scene_screenshot_indices[capture_idx] < capture.times.len()
                && current_time + 1e-4 >= capture.times[self.scene_screenshot_indices[capture_idx]]
            {
                let current_idx = self.scene_screenshot_indices[capture_idx];
                let requested = capture
                    .names
                    .as_ref()
                    .and_then(|names| names.get(current_idx))
                    .and_then(|name| name.as_deref());
                let path = self.resolve_screenshot_path(settings, requested);
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                save_png_fast(image, &path)?;
                self.scene_screenshot_indices[capture_idx] += 1;
            }
        }

        for (capture_idx, capture) in scene.gif_captures.iter().enumerate() {
            while self.scene_gif_indices[capture_idx] < capture.times.len()
                && current_time + 1e-4 >= capture.times[self.scene_gif_indices[capture_idx]]
            {
                self.capture_groups(image, settings, std::slice::from_ref(&capture.name))?;
                self.scene_gif_indices[capture_idx] += 1;
            }
        }

        Ok(())
    }

    fn capture_groups(
        &mut self,
        image: &image::RgbaImage,
        settings: &ExportSettings,
        groups: &[String],
    ) -> Result<()> {
        let Some(output_dir) = self.output_dir.as_ref() else {
            return Ok(());
        };

        for group in groups {
            let group_key = sanitize_stem(group);
            let next_index = self.frame_counts.entry(group_key.clone()).or_insert(0);
            let frame_path = marker_gif_frames_root(output_dir)
                .join(&group_key)
                .join(format!("{group_key}_{:05}.png", *next_index));
            if let Some(parent) = frame_path.parent() {
                fs::create_dir_all(parent)?;
            }
            save_png_fast(image, &frame_path)?;
            *next_index += 1;
        }

        let _ = settings;
        Ok(())
    }

    fn assemble_all(&self, settings: &ExportSettings) -> Result<()> {
        let Some(output_dir) = self.output_dir.as_ref() else {
            return Ok(());
        };
        if self.frame_counts.is_empty() {
            return Ok(());
        }
        if !ffmpeg_available() {
            return Err(anyhow::anyhow!(
                "ffmpeg not found in PATH; marker GIF frames were exported to {}",
                marker_gif_frames_root(output_dir).display()
            ));
        }

        for (group, frame_count) in &self.frame_counts {
            if *frame_count == 0 {
                continue;
            }
            let gif_path = output_dir.join(format!("{group}.gif"));
            assemble_marker_gif(settings, output_dir, group, &gif_path)?;
        }
        Ok(())
    }

    fn resolve_screenshot_path(
        &mut self,
        settings: &ExportSettings,
        requested: Option<&Path>,
    ) -> PathBuf {
        match requested {
            Some(path) => resolve_screenshot_path(settings, path),
            None => {
                let path = settings
                    .output_dir
                    .join("captures")
                    .join(format!("capture_{:05}.png", self.screenshot_count));
                self.screenshot_count += 1;
                path
            }
        }
    }
}

fn assemble_marker_gif(
    settings: &ExportSettings,
    output_dir: &Path,
    group: &str,
    gif_path: &Path,
) -> Result<()> {
    if let Some(parent) = gif_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let pattern = marker_gif_frames_root(output_dir)
        .join(group)
        .join(format!("{group}_%05d.png"));
    let filter = format!(
        "fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
        settings.fps, settings.width
    );

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(pattern)
        .arg("-vf")
        .arg(filter)
        .arg(gif_path)
        .status()
        .context("Failed to spawn ffmpeg for marker GIF assembly")?;

    if !status.success() {
        return Err(anyhow::anyhow!(
            "ffmpeg exited with status {status}; marker GIF frames remain in {}",
            marker_gif_frames_root(output_dir).display()
        ));
    }

    Ok(())
}

fn marker_gif_frames_root(output_dir: &Path) -> PathBuf {
    output_dir.join(".frames")
}

fn resolve_screenshot_path(settings: &ExportSettings, requested: &Path) -> PathBuf {
    if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        settings.output_dir.join(requested)
    }
}

fn save_png_fast(image: &image::RgbaImage, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = fs::File::create(path)?;
    let writer = BufWriter::new(file);
    let encoder = PngEncoder::new_with_quality(writer, CompressionType::Fast, FilterType::NoFilter);
    encoder.write_image(
        image.as_raw(),
        image.width(),
        image.height(),
        ColorType::Rgba8.into(),
    )?;
    Ok(())
}
