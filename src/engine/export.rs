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
use crate::frontend::collection::utility::screenshot_marker::ScreenshotMarker;

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
            clear_color: Vec4::new(0.05, 0.10, 0.15, 1.0),
        }
    }
}

impl ExportSettings {
    pub fn from_scene(scene: &Scene) -> Self {
        Self {
            duration_seconds: infer_duration(scene),
            ..Self::default()
        }
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

        save_requested_screenshots(&image, &mut engine.scene, settings).with_context(|| {
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
    scene: &mut Scene,
    settings: &ExportSettings,
) -> Result<()> {
    let marker_ids: Vec<_> = scene
        .tattvas
        .iter()
        .filter_map(|(id, tattva)| {
            tattva
                .as_any()
                .downcast_ref::<crate::frontend::Tattva<ScreenshotMarker>>()
                .and_then(|marker| marker.state.should_capture().then_some(*id))
        })
        .collect();

    for id in marker_ids {
        let Some(marker) = scene.get_tattva_typed_mut::<ScreenshotMarker>(id) else {
            continue;
        };

        let path = resolve_screenshot_path(settings, &marker.state.output_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        save_png_fast(image, &path)?;
        marker.state.mark_captured();
    }

    Ok(())
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
