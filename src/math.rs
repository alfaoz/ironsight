use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub data: [[f64; 4]; 4],
}

// Vec2 implementations
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len != 0.0 {
            Self::new(self.x / len, self.y / len)
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// Vec3 implementations
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length != 0.0 {
            *self / length
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

// Mat4 implementations
impl Mat4 {
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut m = Self::identity();
        m.data[0][3] = x;
        m.data[1][3] = y;
        m.data[2][3] = z;
        m
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut m = Self::identity();
        m.data[0][0] = x;
        m.data[1][1] = y;
        m.data[2][2] = z;
        m
    }

    pub fn rotation_x(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut m = Self::identity();
        m.data[1][1] = cos;
        m.data[1][2] = -sin;
        m.data[2][1] = sin;
        m.data[2][2] = cos;
        m
    }

    pub fn rotation_y(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut m = Self::identity();
        m.data[0][0] = cos;
        m.data[0][2] = sin;
        m.data[2][0] = -sin;
        m.data[2][2] = cos;
        m
    }

    pub fn rotation_z(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut m = Self::identity();
        m.data[0][0] = cos;
        m.data[0][1] = -sin;
        m.data[1][0] = sin;
        m.data[1][1] = cos;
        m
    }

    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Mat4::new(result)
    }

    pub fn transform_vec3(&self, v: &Vec3) -> Vec3 {
        let x = v.x * self.data[0][0] + v.y * self.data[0][1] + v.z * self.data[0][2] + self.data[0][3];
        let y = v.x * self.data[1][0] + v.y * self.data[1][1] + v.z * self.data[1][2] + self.data[1][3];
        let z = v.x * self.data[2][0] + v.y * self.data[2][1] + v.z * self.data[2][2] + self.data[2][3];
        let w = v.x * self.data[3][0] + v.y * self.data[3][1] + v.z * self.data[3][2] + self.data[3][3];

        if w != 0.0 {
            Vec3::new(x/w, y/w, z/w)
        } else {
            Vec3::new(x, y, z)
        }
    }
}

// Operator implementations for Vec2
impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

// Operator implementations for Vec3
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_vec2_operations() {
        let v1 = Vec2::new(3.0, 4.0);
        let v2 = Vec2::new(1.0, 2.0);

        assert_eq!(v1.length(), 5.0);
        assert_eq!(v1.dot(&v2), 11.0);

        let normalized = v1.normalize();
        assert!((normalized.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let cross = v1.cross(&v2);
        assert_eq!(cross, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_matrix_operations() {
        let translation = Mat4::translation(1.0, 2.0, 3.0);
        let point = Vec3::new(1.0, 1.0, 1.0);

        let transformed = translation.transform_vec3(&point);
        assert_eq!(transformed, Vec3::new(2.0, 3.0, 4.0));

        let rotation = Mat4::rotation_y(PI / 2.0);
        let point = Vec3::new(1.0, 0.0, 0.0);
        let rotated = rotation.transform_vec3(&point);

        assert!((rotated.x - 0.0).abs() < 1e-10);
        assert!((rotated.z + 1.0).abs() < 1e-10);
    }
}
