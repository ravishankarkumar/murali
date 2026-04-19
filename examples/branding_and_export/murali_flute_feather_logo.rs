use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::export::{ExportSettings, export_scene};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::composite::{MuraliLogoMark, MuraliLogoPalette};

fn build_scene(palette: MuraliLogoPalette) -> Scene {
    let mut scene = Scene::new();

    let murali_logo = MuraliLogoMark::new().with_palette(palette);

    let murali_logo_id = scene.add_tattva(murali_logo, Vec3::ZERO);

    let mut timeline = Timeline::new();
    timeline
        .animate(murali_logo_id)
        .at(0.0)
        .for_duration(1.8)
        .ease(Ease::InOutCubic)
        .move_to(Vec3::new(-1.2, 0.4, 0.0))
        .spawn();

    timeline
        .animate(murali_logo_id)
        .at(0.0)
        .for_duration(1.8)
        .ease(Ease::InOutCubic)
        .scale_to(Vec3::splat(0.72))
        .spawn();

    scene.set_timeline("main", timeline);

    scene.camera_mut().position = CAMERA_DEFAULT_POS;
    scene
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--export") {
        let export_dir = "render_output/logo_exports";
        let light_settings = ExportSettings {
            duration_seconds: 0.1,
            output_dir: export_dir.into(),
            basename: "murali_logo_light".to_string(),
            video_path: None,
            gif_path: None,
            clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            ..ExportSettings::default()
        };
        export_scene(build_scene(MuraliLogoPalette::Light), &light_settings)?;

        let dark_settings = ExportSettings {
            basename: "murali_logo_dark".to_string(),
            ..light_settings.clone()
        };
        return export_scene(build_scene(MuraliLogoPalette::Dark), &dark_settings);
    }

    App::new()?
        .with_scene(build_scene(MuraliLogoPalette::Dark))
        .run_app()
}
