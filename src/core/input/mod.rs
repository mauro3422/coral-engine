//! Input System - Coral Engine v0.5.0
//! Modular input handling with context-aware actions

use winit::keyboard::KeyCode;

pub mod action_map;
pub mod action_state;
pub mod actions;
pub mod context;
pub mod context_map;
pub mod input_manager;
pub mod key_modifiers;
pub mod keymap_persist;

pub use action_map::ActionMap;
pub use action_state::ActionState;
pub use actions::InputAction;
pub use context::{ActionCategory, InputContext};
pub use context_map::ContextActionMap;
pub use input_manager::InputManager;
pub use key_modifiers::{KeyCombo, KeyModifiers};
pub use keymap_persist::KeymapManager;

// Use centralized constants
#[allow(unused_imports)]
pub use crate::common::constants::{MOUSE_CAPTURE_THRESHOLD, MOUSE_SENSITIVITY};

// Alias for backwards compatibility
pub type InputState = InputManager;

/// Input Handler trait for testability
pub trait InputHandler: Send {
    fn handle_key_down(&mut self, key: KeyCode);
    fn handle_key_up(&mut self, key: KeyCode);
    fn is_action_active(&self, action: InputAction) -> bool;
}

impl InputHandler for InputManager {
    fn handle_key_down(&mut self, key: KeyCode) {
        InputManager::key_down(self, key);
    }

    fn handle_key_up(&mut self, key: KeyCode) {
        InputManager::key_up(self, key);
    }

    fn is_action_active(&self, action: InputAction) -> bool {
        InputManager::is_action_active(self, action)
    }
}
