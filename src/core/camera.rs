//! Camera module - Coral Engine
//! Standard camera with configurable parameters

use crate::common::constants::{
    CAMERA_ZOOM_SPEED, DEFAULT_CAMERA_FOV, DEFAULT_CAMERA_PITCH, DEFAULT_CAMERA_POSITION,
    DEFAULT_CAMERA_SENSITIVITY, DEFAULT_CAMERA_SPEED, DEFAULT_CAMERA_YAW, FAR_PLANE,
    MAX_CAMERA_FOV, MAX_CAMERA_PITCH, MIN_CAMERA_FOV, MIN_CAMERA_PITCH, NEAR_PLANE,
};
use cgmath::{perspective, Angle, Deg, InnerSpace, Matrix4, Point3, Vector3};
use std::default::Default;

/// Camera - viewpoint for 3D rendering
#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub pitch: f32,
    pub yaw: f32,
    pub fov: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    /// Create new camera with defaults
    pub fn new() -> Self {
        Self {
            position: Point3::new(
                DEFAULT_CAMERA_POSITION[0],
                DEFAULT_CAMERA_POSITION[1],
                DEFAULT_CAMERA_POSITION[2],
            ),
            pitch: DEFAULT_CAMERA_PITCH,
            yaw: DEFAULT_CAMERA_YAW,
            fov: DEFAULT_CAMERA_FOV,
            speed: DEFAULT_CAMERA_SPEED,
            sensitivity: DEFAULT_CAMERA_SENSITIVITY,
        }
    }

    // Getters
    pub fn position(&self) -> Point3<f32> {
        self.position
    }
    pub fn pitch(&self) -> f32 {
        self.pitch
    }
    pub fn yaw(&self) -> f32 {
        self.yaw
    }
    pub fn fov(&self) -> f32 {
        self.fov
    }
    pub fn speed(&self) -> f32 {
        self.speed
    }
    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    // Setters with validation
    pub fn set_position(&mut self, pos: Point3<f32>) {
        self.position = pos;
    }
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(MIN_CAMERA_PITCH, MAX_CAMERA_PITCH);
    }
    pub fn set_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov.clamp(MIN_CAMERA_FOV, MAX_CAMERA_FOV);
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.1);
    }
    pub fn set_sensitivity(&mut self, sens: f32) {
        self.sensitivity = sens.max(0.01);
    }

    /// Create camera with custom config
    pub fn with_config(
        position: Point3<f32>,
        pitch: f32,
        yaw: f32,
        fov: f32,
        speed: f32,
        sensitivity: f32,
    ) -> Self {
        Self {
            position,
            pitch: pitch.clamp(MIN_CAMERA_PITCH, MAX_CAMERA_PITCH),
            yaw,
            fov: fov.clamp(MIN_CAMERA_FOV, MAX_CAMERA_FOV),
            speed: speed.max(0.1),
            sensitivity: sensitivity.max(0.01),
        }
    }

    /// Get view-projection matrix
    pub fn view_projection(&self, aspect_ratio: f32) -> Matrix4<f32> {
        self.projection_matrix(aspect_ratio) * self.view_matrix()
    }

    /// Get view matrix
    pub fn view_matrix(&self) -> Matrix4<f32> {
        let pitch = Deg(self.pitch);
        let yaw = Deg(self.yaw);

        let front = Vector3::new(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos(),
        )
        .normalize();

        let target = self.position + front;
        Matrix4::look_at_rh(self.position, target, Vector3::unit_y())
    }

    /// Get projection matrix
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
        )
        .normalize();

        self.position += forward * delta * CAMERA_ZOOM_SPEED;
    }

    /// Rotate camera with delta input
    pub fn rotate(&mut self, delta_x: f64, delta_y: f64) {
        self.yaw -= (delta_x * self.sensitivity as f64) as f32;
        self.pitch -= (delta_y * self.sensitivity as f64) as f32;
        self.pitch = self.pitch.clamp(MIN_CAMERA_PITCH, MAX_CAMERA_PITCH);
    }

    /// Move forward
    pub fn move_forward(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(yaw.sin(), 0.0, yaw.cos()).normalize();
        self.position += direction * self.speed * delta_time;
    }

    /// Move backward
    pub fn move_backward(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(yaw.sin(), 0.0, yaw.cos()).normalize();
        self.position -= direction * self.speed * delta_time;
    }

    /// Move left
    pub fn move_left(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(yaw.cos(), 0.0, -yaw.sin()).normalize();
        self.position += direction * self.speed * delta_time;
    }

    /// Move right
    pub fn move_right(&mut self, delta_time: f32) {
        let yaw = Deg(self.yaw);
        let direction = Vector3::new(-yaw.cos(), 0.0, yaw.sin()).normalize();
        self.position += direction * self.speed * delta_time;
    }
}
