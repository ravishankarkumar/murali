use murali::prelude::*;
use murali::sangh::shapes::Circle; // Fix the Circle import
use murali::sangh::axes::Axes;
use murali::app::App;
use std::sync::Arc;
use winit::window::Window;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new()?;

    // Create your math scene
    let mut scene = Scene::new();
    let circle = Sangh::new(1, Circle::new(0.5));
    scene.add(circle);
    //
    let mut scene = Scene::new();
    let mut axes = Axes::new((-10.0, 10.0), (-5.0, 5.0));
    axes.x_step = 2.0;

    scene.add_sangh(axes);
    //

    let mut scene = Scene::new();

    app.with_scene(scene).run_app()
}
