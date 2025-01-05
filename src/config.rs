pub struct Config {
    pub window_width: usize,
    pub window_height: usize,
    pub window_title: &'static str,
    pub clear_color: (u8, u8, u8, u8),
    pub movement_speed: f64,
    pub rotation_speed: f64,
    pub fov: f64,
    pub near_plane: f64,
    pub far_plane: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            window_title: "sgr-rs",
            clear_color: (0, 0, 0, 255),
            movement_speed: 5.0,
            rotation_speed: 2.0,
            fov: 60.0,
            near_plane: 0.1,
            far_plane: 1000.0,
        }
    }
}
