// Camera module - Coral Engine
// Standard camera with configurable parameters

use cgmath::{perspective, Angle, Deg, InnerSpace, Matrix4, Point3, Vector3};

// Camera constants - canonical defaults
pub const DEFAULT_POSITION: [f32; 3] = [20.0, 20.0, 60.0];
pub const DEFAULT_PITCH: f32 = -25.0;
pub const DEFAULT_YAW: f32 = 180.0;
pub const DEFAULT_FOV: f32 = 60.0;
pub const DEFAULT_SPEED: f32 = 15.0;
pub const DEFAULT_SENSITIVITY: f32 = 0.15;

pub const MIN_PITCH: f32 = -89.0;
pub const MAX_PITCH: f32 = 89.0;
pub const MIN_FOV: f32 = 30.0;
pub const MAX_FOV: f32 = 120.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 500.0;

pub struct Camera {
    pub position: Point3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub fov: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Point3::new(DEFAULT_POSITION[0], DEFAULT_POSITION[1], DEFAULT_POSITION[2]),
            pitch: DEFAULT_PITCH,
            yaw: DEFAULT_YAW,
            fov: DEFAULT_FOV,
            speed: DEFAULT_SPEED,
            sensitivity: DEFAULT_SENSITIVITY,
        }
    }

    /// Create camera with custom defaults
    pub fn with_config(position: Point3<f32>, pitch: f32, yaw: f32, fov: f32, speed: f32, sensitivity: f32) -> Self {
        Self {
            position,
            pitch: pitch.clamp(MIN_PITCH, MAX_PITCH),
            yaw,
            fov: fov.clamp(MIN_FOV, MAX_FOV),
            speed,
            sensitivity,
        }
    }

    pub fn view_projection(&self, aspect_ratio: f32) -> Matrix4<f32> {
        self.projection_matrix(aspect_ratio) * self.view_matrix()
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let pitch = Deg(self.pitch);
        let yaw = Deg(self.yaw);

        let front = Vector3::new(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        ).normalize();

        let target = self.position + front;
        Matrix4::look_at_rh(self.position, target, Vector3::unit_y())
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32> {
        perspective(Deg(self.fov), aspect_ratio, NEAR_PLANE, FAR_PLANE)
    }

    /// Zoom: move camera along view direction
    pub fn zoom(&mut self, delta: f32) {
        let pitch = Deg(self.pitch);
        let yaw = Deg(self.yaw);

        let forward = Vector3::new(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        ).normalize();

        self.position += forward * delta * 5.0;
    }

    /// Rotate camera with delta input
    pub fn rotate(&mut self, delta_x: f64, delta_y: f64) {
        self.yaw -= (delta_x * self.sensitivity as f64) as f32;
        self.pitch -= (delta_y * self.sensitivity as f64) as f32;
        self.pitch = self.pitch.clamp(MIN_PITCH, MAX_PITCH);
    }

    // Movement methods
    pub fn move_forward(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(yaw.sin(), 0.0, yaw.cos()).normalize();
        self.position += direction * self.speed * delta_time;
    }

    pub fn move_backward(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(yaw.sin(), 0.0, yaw.cos()).normalize();
        self.position -= direction * self.speed * delta_time;
    }

    pub fn move_left(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(yaw.cos(), 0.0, -yaw.sin()).normalize();
        self.position += direction * self.speed * delta_time;
    }

    pub fn move_right(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(-yaw.cos(), 0.0, yaw.sin()).normalize();
        self.position += direction * self.speed * delta_time;
    }
}

impl Default for Camera {
    fn default() -> Self { Self::new() }
}
