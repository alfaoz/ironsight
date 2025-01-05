use crate::math::{Vec2, Vec3, Mat4};
use crate::geometry::{Mesh, Vertex};
use crate::camera::Camera;
use crate::rasterizer::{Rasterizer, Color};

pub struct Renderer {
    rasterizer: Rasterizer,
    width: usize,
    height: usize,
    clear_color: Color,
    wireframe_mode: bool,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            rasterizer: Rasterizer::new(width, height),
            width,
            height,
            clear_color: Color::black(),
            wireframe_mode: false,
        }
    }

    pub fn clear(&mut self) {
        self.rasterizer.clear(self.clear_color);
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn toggle_wireframe(&mut self) {
        self.wireframe_mode = !self.wireframe_mode;
    }

    pub fn get_buffer(&self) -> &[u32] {
        self.rasterizer.get_color_buffer()
    }

    pub fn render_mesh(&mut self, mesh: &Mesh, transform: &Mat4, camera: &Camera) {
        let view_projection = camera.get_view_projection_matrix();
        let model_view_projection = view_projection.multiply(transform);

        // Transform vertices
        let transformed_vertices: Vec<Vec3> = mesh.vertices.iter()
            .map(|v| model_view_projection.transform_vec3(&v.position))
            .collect();

        // Project to screen space
        let screen_vertices: Vec<Vec2> = transformed_vertices.iter()
            .map(|v| self.to_screen_space(v))
            .collect();

        // Draw triangles
        for face in &mesh.faces {
            let v0 = screen_vertices[face.vertices[0]];
            let v1 = screen_vertices[face.vertices[1]];
            let v2 = screen_vertices[face.vertices[2]];

            // Always draw wireframe for debugging
            self.rasterizer.draw_triangle_wireframe(
                v0, v1, v2,
                Color::new(255, 255, 255, 255)
            );
        }
    }
    fn to_screen_space(&self, v: &Vec3) -> Vec2 {
        // Proper perspective divide
        if v.z.abs() < 0.001 {
            return Vec2::new(0.0, 0.0);
        }

        let inv_z = 1.0 / v.z;
        let x = (v.x * inv_z + 1.0) * 0.5 * self.width as f64;
        let y = (-v.y * inv_z + 1.0) * 0.5 * self.height as f64;

        Vec2::new(x, y)
    }

    fn is_face_visible(&self, v0: Vec2, v1: Vec2, v2: Vec2) -> bool {
        // Calculate signed area of triangle
        let area = (v1.x - v0.x) * (v2.y - v0.y) - (v2.x - v0.x) * (v1.y - v0.y);
        area > 0.0
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_creation() {
        let renderer = Renderer::new(800, 600);
        assert!(!renderer.wireframe_mode);
    }

    #[test]
    fn test_screen_space_conversion() {
        let renderer = Renderer::new(800, 600);
        let point = Vec3::new(0.0, 0.0, 1.0);
        let screen_point = renderer.to_screen_space(&point);
        assert_eq!(screen_point.x as i32, 400);
        assert_eq!(screen_point.y as i32, 300);
    }
}
