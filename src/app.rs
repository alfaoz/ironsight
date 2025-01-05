use std::time::Instant;
use minifb::{Window, WindowOptions, Key};

use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::shape_factory::ShapeFactory;
use crate::math::Vec3;


pub struct Application {
    window: Window,
    renderer: Renderer,
    scene: Scene,
    camera: Camera,
    last_frame: Instant,
    delta_time: f64,
}

impl Application {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        ).expect("Failed to create window");

        let renderer = Renderer::new(width, height);
        let scene = Scene::new();
        let mut camera = Camera::new(width as f64, height as f64);

        // Position camera to see the scene
        camera.set_position(Vec3::new(0.0, 0.0, -5.0));
        camera.look_at(Vec3::new(0.0, 0.0, 0.0));

        Self {
            window,
            renderer,
            scene,
            camera,
            last_frame: Instant::now(),
            delta_time: 0.0,
        }
    }

    pub fn run(&mut self) {
        self.setup_scene();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.update();
            self.render();

            // Update window with rendered frame
            self.window.update_with_buffer(
                self.renderer.get_buffer(),
                self.renderer.width(),
                self.renderer.height(),
            ).unwrap();
        }
    }

    fn setup_scene(&mut self) {
        // Create a smaller cube for testing
        let cube_mesh = ShapeFactory::create_cube(2.0);
        let cube_id = self.scene.create_mesh_node("cube".to_string(), cube_mesh);

        // Position the cube in view
        if let Some(node) = self.scene.get_node_mut(cube_id) {
            node.transform.set_position(Vec3::new(0.0, 0.0, 0.0));
            node.transform.set_scale(Vec3::new(1.0, 1.0, 1.0));
        }
    }

    fn update(&mut self) {
        // Update delta time
        let current_time = Instant::now();
        self.delta_time = (current_time - self.last_frame).as_secs_f64();
        self.last_frame = current_time;

        self.handle_input();
        self.update_scene();
    }

    fn handle_input(&mut self) {
        let movement_speed = 3.0 * self.delta_time;
        let rotation_speed = 2.0 * self.delta_time;

        // Camera movement
        if self.window.is_key_down(Key::W) {
            self.camera.move_forward(movement_speed);
        }
        if self.window.is_key_down(Key::S) {
            self.camera.move_forward(-movement_speed);
        }
        if self.window.is_key_down(Key::A) {
            self.camera.move_right(-movement_speed);
        }
        if self.window.is_key_down(Key::D) {
            self.camera.move_right(movement_speed);
        }
        if self.window.is_key_down(Key::Q) {
            self.camera.rotate_horizontal(-rotation_speed);
        }
        if self.window.is_key_down(Key::E) {
            self.camera.rotate_horizontal(rotation_speed);
        }
        if self.window.is_key_down(Key::R){
            self.camera.rotate_vertical(-rotation_speed*0.6);
        }
        if self.window.is_key_down(Key::F){
            self.camera.rotate_vertical(rotation_speed*0.6);
        }

        // Toggle wireframe mode
        if self.window.is_key_pressed(Key::O, minifb::KeyRepeat::No) {
            self.renderer.toggle_wireframe();
        }
    }

    fn update_scene(&mut self) {
        self.scene.update_transforms();

        if let Some(node_id) = self.scene.find_node_by_name("cube") {
            if let Some(node) = self.scene.get_node_mut(node_id) {
                // Rotate the cube
                let current_rotation = node.transform.rotation;
                node.transform.set_rotation(Vec3::new(
                    current_rotation.x + self.delta_time,
                    current_rotation.y + self.delta_time,
                    current_rotation.z + self.delta_time,
                ));
            }
        }
    }

    fn render(&mut self) {
        self.renderer.clear();
        self.camera.update();

        self.scene.traverse_visible(|node| {
            if let Some(mesh) = &node.mesh {
                self.renderer.render_mesh(mesh, &node.transform.world_matrix, &self.camera);
            }
        });
    }

}

impl Drop for Application {
    fn drop(&mut self) {
    }
}
