mod app;
mod camera;
mod geometry;
mod math;
mod renderer;
mod rasterizer;
mod scene;
mod shape_factory;
mod config;

use app::Application;

fn main() {
    println!("Starting application...");
    let mut app = Application::new(800, 600, "sgr-rs test");
    println!("Application created, running...");
    app.run();
}

