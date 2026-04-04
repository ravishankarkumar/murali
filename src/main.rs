use glam::{Vec3, Vec4};
use murali::App;
use murali::engine::doctor::DoctorReport;
use murali::engine::scene::Scene;
use murali::frontend::Tattva;
use murali::frontend::collection::primitives::square::Square;

fn main() -> anyhow::Result<()> {
    if matches!(std::env::args().nth(1).as_deref(), Some("doctor")) {
        let report = DoctorReport::gather();
        println!("{}", report.render_text());
        return Ok(());
    }

    let scene = {
        let mut scene = Scene::new();

        // 1. Pure semantic object
        let square = Square::new(2.0, Vec4::new(1.0, 0.0, 0.0, 1.0));

        // 2. Wrap it into a Tattva
        let square_tattva = Tattva::new(0, square); // id will be overwritten by Scene

        // 3. Add to scene
        let id = scene.add(square_tattva);

        // 4. Mutate shared props
        if let Some(t) = scene.get_tattva_any_mut(id) {
            let mut props = t.props().write();
            props.position = Vec3::new(0.0, 0.0, -5.0);
        }

        // 5. Camera
        scene.camera_mut().position = Vec3::new(0.0, 0.0, 10.0);

        scene
    };

    App::new()?.with_scene(scene).run_app()
}
