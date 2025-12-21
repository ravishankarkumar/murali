use murali::app::App;
use murali::prelude::*;
use murali::sangh::axes::Axes;
use murali::sangh::shapes::Circle; // Fix the Circle import
use std::sync::Arc;
use winit::window::Window;
use murali::tattva::props::TattvaProps;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut app = App::new()?;

    // // Create your math scene
    // let mut scene = Scene::new();
    // let circle = Sangh::new(1, Circle::new(0.5));
    // scene.add(circle);
    // //
    // let mut scene = Scene::new();
    // let mut axes = Axes::new((-10.0, 10.0), (-5.0, 5.0));
    // axes.x_step = 2.0;

    // scene.add_sangh(axes);
    // //

    // let mut scene = Scene::new();

    // app.with_scene(scene).run_app()

    let mut app = App::new()?;
    let mut scene = Scene::new();

    // 1. Create a Square Tattva (or Circle)
    // We use DrawableProps to move it into view
    let square = murali::tattva::square::Square::new(TattvaProps::default()); // Assuming this exists

    // 2. Spawn it so it's added to scene.drawables
    scene.spawn(
        square,
        DrawableProps::default().at(glam::Vec3::new(0.0, 0.0, -5.0)),
    );

    // 3. Move the camera back slightly just in case
    scene.camera.position = glam::Vec3::new(0.0, 0.0, 10.0);

    app.with_scene(scene).run_app()
}
