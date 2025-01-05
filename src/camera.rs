use crate::math::{Vec3, Mat4};
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Camera {
    // Basic camera properties
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,

    // Projection properties
    pub fov: f64,        // Field of view in radians
    pub aspect_ratio: f64,
    pub near: f64,       // Near clipping plane
    pub far: f64,        // Far clipping plane

    // Derived matrices
    view_matrix: Mat4,
    projection_matrix: Mat4,

    // Camera movement properties
    pub movement_speed: f64,
    pub rotation_speed: f64,
}

impl Camera {
    pub fn new(width: f64, height: f64) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, -5.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 60.0 * PI / 180.0,
            aspect_ratio: width / height,
            near: 0.1,
            far: 100.0,
            view_matrix: Mat4::identity(),
            projection_matrix: Mat4::identity(),
            movement_speed: 5.0,
            rotation_speed: 2.0,
        };
        camera.update_matrices();
        camera
    }


    pub fn update_matrices(&mut self) {
        self.update_view_matrix();
        self.update_projection_matrix();
    }

    fn update_view_matrix(&mut self) {
        // Calculate camera basis vectors
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        // Create rotation matrix
        let mut rotation = Mat4::identity();
        rotation.data[0][0] = right.x;
        rotation.data[0][1] = right.y;
        rotation.data[0][2] = right.z;
        rotation.data[1][0] = up.x;
        rotation.data[1][1] = up.y;
        rotation.data[1][2] = up.z;
        rotation.data[2][0] = -forward.x;
        rotation.data[2][1] = -forward.y;
        rotation.data[2][2] = -forward.z;

        // Create translation matrix
        let translation = Mat4::translation(-self.position.x, -self.position.y, -self.position.z);

        // Combine matrices
        self.view_matrix = rotation.multiply(&translation);
    }

    fn update_projection_matrix(&mut self) {
        let f = 1.0 / (self.fov / 2.0).tan();
        let range_inv = 1.0 / (self.near - self.far);

        self.projection_matrix = Mat4::new([
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (self.far + self.near) * range_inv, -1.0],
            [0.0, 0.0, 2.0 * self.far * self.near * range_inv, 0.0],
        ]);
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        self.view_matrix.clone()
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix.clone()
    }

    pub fn get_view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix.multiply(&self.view_matrix)
    }

    // Camera movement methods
    pub fn move_forward(&mut self, amount: f64) {
        let forward = (self.target - self.position).normalize();
        self.position = self.position + forward * (amount * self.movement_speed);
        self.target = self.target + forward * (amount * self.movement_speed);
        self.update_matrices();
    }

    pub fn move_right(&mut self, amount: f64) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        self.position = self.position + right * (amount * self.movement_speed);
        self.target = self.target + right * (amount * self.movement_speed);
        self.update_matrices();
    }

    pub fn move_up(&mut self, amount: f64) {
        let up = self.up.normalize();
        self.position = self.position + up * (amount * self.movement_speed);
        self.target = self.target + up * (amount * self.movement_speed);
        self.update_matrices();
    }

    pub fn rotate_horizontal(&mut self, angle: f64) {
        let forward = (self.target - self.position).normalize();
        let _right = forward.cross(&self.up).normalize();

        let rotation = Mat4::rotation_y(angle * self.rotation_speed);
        let rotated_forward = rotation.transform_vec3(&forward);

        self.target = self.position + rotated_forward;
        self.update_matrices();
    }

    pub fn rotate_vertical(&mut self, angle: f64) {
        let forward = (self.target - self.position).normalize();
        let _right = forward.cross(&self.up).normalize();

        let rotation = Mat4::rotation_x(angle * self.rotation_speed);
        let rotated_forward = rotation.transform_vec3(&forward);

        self.target = self.position + rotated_forward;
        self.update_matrices();
    }

    // Frustum culling methods
    pub fn get_frustum_planes(&self) -> [Vec4; 6] {
        let vp_matrix = self.get_view_projection_matrix();
        let m = &vp_matrix.data;

        // Extract frustum planes from view-projection matrix
        // Each plane is represented as Ax + By + Cz + D = 0
        [
            // Left plane
            Vec4::new(
                m[0][3] + m[0][0],
                m[1][3] + m[1][0],
                m[2][3] + m[2][0],
                m[3][3] + m[3][0]
            ).normalize(),
            // Right plane
            Vec4::new(
                m[0][3] - m[0][0],
                m[1][3] - m[1][0],
                m[2][3] - m[2][0],
                m[3][3] - m[3][0]
            ).normalize(),
            // Bottom plane
            Vec4::new(
                m[0][3] + m[0][1],
                m[1][3] + m[1][1],
                m[2][3] + m[2][1],
                m[3][3] + m[3][1]
            ).normalize(),
            // Top plane
            Vec4::new(
                m[0][3] - m[0][1],
                m[1][3] - m[1][1],
                m[2][3] - m[2][1],
                m[3][3] - m[3][1]
            ).normalize(),
            // Near plane
            Vec4::new(
                m[0][3] + m[0][2],
                m[1][3] + m[1][2],
                m[2][3] + m[2][2],
                m[3][3] + m[3][2]
            ).normalize(),
            // Far plane
            Vec4::new(
                m[0][3] - m[0][2],
                m[1][3] - m[1][2],
                m[2][3] - m[2][2],
                m[3][3] - m[3][2]
            ).normalize(),
        ]
    }
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn look_at(&mut self, target: Vec3) {
        self.target = target;
    }

    pub fn update(&mut self) {
        // Update view and projection matrices
        self.update_matrices();
    }

    pub fn rotate_y(&mut self, angle: f64) {
        // Implement rotation around Y axis
        let cos = angle.cos();
        let sin = angle.sin();
        let relative_pos = self.position - self.target;

        self.position.x = self.target.x + (relative_pos.x * cos - relative_pos.z * sin);
        self.position.z = self.target.z + (relative_pos.x * sin + relative_pos.z * cos);
    }
}

// Helper struct for frustum planes
#[derive(Debug, Clone, Copy)]
struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vec4 {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if length != 0.0 {
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
                w: self.w / length,
            }
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_creation() {
        let camera = Camera::new(800.0, 600.0);
        assert!((camera.aspect_ratio - 800.0/600.0).abs() < 1e-10);
    }

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::new(800.0, 600.0);
        let initial_position = camera.position;

        camera.move_forward(1.0);
        assert!(camera.position.z > initial_position.z);

        camera.move_right(1.0);
        assert!(camera.position.x != initial_position.x);
    }

    #[test]
    fn test_camera_rotation() {
        let mut camera = Camera::new(800.0, 600.0);
        let initial_target = camera.target;

        camera.rotate_horizontal(PI / 2.0);
        assert!(camera.target.x != initial_target.x);
    }

    #[test]
    fn test_view_matrix() {
        let camera = Camera::new(800.0, 600.0);
        let view_matrix = camera.get_view_matrix();
        assert!(view_matrix.data[3][3] == 1.0);
    }
}
