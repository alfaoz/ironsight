use crate::math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }
}

pub struct Rasterizer {
    width: usize,
    height: usize,
    color_buffer: Vec<u32>,
    depth_buffer: Vec<f64>,
}

impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![0; width * height],
            depth_buffer: vec![f64::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self, color: Color) {
        let clear_color = color.to_u32();
        self.color_buffer.fill(clear_color);
        self.depth_buffer.fill(f64::INFINITY);
    }

    pub fn get_color_buffer(&self) -> &[u32] {
        &self.color_buffer
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, z: f64, color: Color) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }

        let index = (y as usize) * self.width + (x as usize);

        // Depth test
        if z < self.depth_buffer[index] {
            self.depth_buffer[index] = z;
            self.color_buffer[index] = color.to_u32();
        }
    }

    pub fn draw_line(&mut self, start: Vec2, end: Vec2, color: Color) {
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        // Basic line clipping
        if (x0 < 0 && x1 < 0) || (x0 >= self.width as i32 && x1 >= self.width as i32) ||
            (y0 < 0 && y1 < 0) || (y0 >= self.height as i32 && y1 >= self.height as i32) {
            return;
        }

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                self.set_pixel(x, y, 0.0, color);
            }

            if x == x1 && y == y1 { break; }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
    pub fn draw_triangle(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        // Compute bounding box
        let min_x = v0.x.min(v1.x).min(v2.x).max(0.0) as i32;
        let min_y = v0.y.min(v1.y).min(v2.y).max(0.0) as i32;
        let max_x = v0.x.max(v1.x).max(v2.x).min(self.width as f64 - 1.0) as i32;
        let max_y = v0.y.max(v1.y).max(v2.y).min(self.height as f64 - 1.0) as i32;

        // Edge functions
        let edge = |a: Vec2, b: Vec2, c: Vec2| -> f64 {
            (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
        };

        // Triangle area
        let area = edge(v0, v1, v2);
        if area.abs() < 1e-8 {
            return; // Degenerate triangle
        }

        // Scan through bounding box
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f64 + 0.5, y as f64 + 0.5);

                // Compute barycentric coordinates
                let w0 = edge(v1, v2, p);
                let w1 = edge(v2, v0, p);
                let w2 = edge(v0, v1, p);

                // Check if point is inside triangle
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let b0 = w0 / area;
                    let b1 = w1 / area;
                    let b2 = w2 / area;

                    // Interpolate z value
                    let z = b0 * v0.x + b1 * v1.x + b2 * v2.x;
                    self.set_pixel(x, y, z, color);
                }
            }
        }
    }

    pub fn draw_triangle_wireframe(&mut self, v0: Vec2, v1: Vec2, v2: Vec2, color: Color) {
        self.draw_line(v0, v1, color);
        self.draw_line(v1, v2, color);
        self.draw_line(v2, v0, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let color = Color::new(255, 128, 64, 255);
        let u32_color = color.to_u32();
        assert_eq!(u32_color & 0xFF, 255);
        assert_eq!((u32_color >> 8) & 0xFF, 128);
        assert_eq!((u32_color >> 16) & 0xFF, 64);
        assert_eq!((u32_color >> 24) & 0xFF, 255);
    }

    #[test]
    fn test_rasterizer_creation() {
        let rasterizer = Rasterizer::new(800, 600);
        assert_eq!(rasterizer.color_buffer.len(), 800 * 600);
        assert_eq!(rasterizer.depth_buffer.len(), 800 * 600);
    }

    #[test]
    fn test_pixel_setting() {
        let mut rasterizer = Rasterizer::new(800, 600);
        let color = Color::white();
        rasterizer.set_pixel(100, 100, 0.0, color);
        assert_eq!(rasterizer.color_buffer[100 * 800 + 100], color.to_u32());
    }
}
