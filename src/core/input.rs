// Input handling module - Coral Engine
use std::collections::HashSet;
use winit::keyboard::KeyCode;

// Input constants
pub const MOUSE_SENSITIVITY: f32 = 0.15;
pub const MOUSE_CAPTURE_THRESHOLD: f64 = 1.0;

pub struct InputState {
    pub keys_pressed: HashSet<KeyCode>,
    pub mouse_delta: (f64, f64),
    pub mouse_captured: bool,
    pub cursor_pos: Option<(f64, f64)>,
    pub prev_cursor_pos: Option<(f64, f64)>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            mouse_delta: (0.0, 0.0),
            mouse_captured: false,
            cursor_pos: None,
            prev_cursor_pos: None,
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn key_down(&mut self, key: KeyCode) {
        self.keys_pressed.insert(key);
    }

    pub fn key_up(&mut self, key: KeyCode) {
        self.keys_pressed.remove(&key);
    }
}

impl Default for InputState {
    fn default() -> Self { Self::new() }
}
