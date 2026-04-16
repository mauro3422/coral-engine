// Input Manager - Coral Engine
// Main input handling interface with context-aware action system

use super::action_state::ActionState;
use super::actions::InputAction;
use super::context::InputContext;
use super::context_map::ContextActionMap;
use super::key_modifiers::{KeyCombo, KeyModifiers};
use std::collections::HashSet;
use winit::event::MouseButton;
use winit::keyboard::KeyCode;

pub struct InputManager {
    pub keys_pressed: HashSet<KeyCode>,
    pub modifiers: KeyModifiers,
    pub context_map: ContextActionMap,
    action_state: ActionState,
    pub mouse_delta: (f64, f64),
    pub mouse_captured: bool,
    pub cursor_pos: Option<(f64, f64)>,
    pub prev_cursor_pos: Option<(f64, f64)>,
    pub mouse_buttons: HashSet<MouseButton>,
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            modifiers: KeyModifiers::none(),
            context_map: ContextActionMap::blender_style(),
            action_state: ActionState::new(),
            mouse_delta: (0.0, 0.0),
            mouse_captured: false,
            cursor_pos: None,
            prev_cursor_pos: None,
            mouse_buttons: HashSet::new(),
        }
    }

    // === Context Management ===

    pub fn active_context(&self) -> InputContext {
        self.context_map.active_context()
    }

    pub fn set_context(&mut self, context: InputContext) {
        self.context_map.set_active_context(context);
    }

    // === Key Handling ===

    pub fn key_down(&mut self, key: KeyCode) {
        self.keys_pressed.insert(key);
        self.update_modifiers();

        let combo = KeyCombo::with_modifiers(self.modifiers, key);

        if let Some(action) = self.context_map.get_active_map().action_for(&combo) {
            self.action_state.press(action);
        }
    }

    pub fn key_up(&mut self, key: KeyCode) {
        self.keys_pressed.remove(&key);
        self.update_modifiers();

        let combo = KeyCombo::with_modifiers(self.modifiers, key);

        if let Some(action) = self.context_map.get_active_map().action_for(&combo) {
            self.action_state.release(action);
        }
    }

    fn update_modifiers(&mut self) {
        self.modifiers = KeyModifiers {
            ctrl: self.keys_pressed.contains(&KeyCode::ControlLeft)
                || self.keys_pressed.contains(&KeyCode::ControlRight),
            shift: self.keys_pressed.contains(&KeyCode::ShiftLeft)
                || self.keys_pressed.contains(&KeyCode::ShiftRight),
            alt: self.keys_pressed.contains(&KeyCode::AltLeft)
                || self.keys_pressed.contains(&KeyCode::AltRight),
            logo: false,
        };
    }

    // === Mouse Handling ===

    pub fn mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_buttons.insert(button);
    }

    pub fn mouse_button_up(&mut self, button: MouseButton) {
        self.mouse_buttons.remove(&button);
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    pub fn update_cursor(&mut self, pos: (f64, f64)) {
        if self.mouse_captured {
            if let Some(prev) = self.prev_cursor_pos {
                self.mouse_delta.0 += pos.0 - prev.0;
                self.mouse_delta.1 += pos.1 - prev.1;
            }
            self.prev_cursor_pos = Some(pos);
        }
        self.cursor_pos = Some(pos);
    }

    pub fn set_mouse_captured(&mut self, captured: bool) {
        self.mouse_captured = captured;
        if !captured {
            self.mouse_delta = (0.0, 0.0);
            self.prev_cursor_pos = None;
        }
    }

    pub fn is_mouse_captured(&self) -> bool {
        self.mouse_captured
    }

    // === Action Queries ===

    pub fn is_action_active(&self, action: InputAction) -> bool {
        self.action_state.is_active(action)
    }

    pub fn is_action_just_pressed(&self, action: InputAction) -> bool {
        self.action_state.is_just_pressed(action)
    }

    pub fn is_action_just_released(&self, action: InputAction) -> bool {
        self.action_state.is_just_released(action)
    }

    // === Frame Management ===

    pub fn clear_frame(&mut self) {
        self.action_state.clear_frame();
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn get_mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn get_cursor_pos(&self) -> Option<(f64, f64)> {
        self.cursor_pos
    }

    // === Keymap Management ===

    pub fn get_context_map(&self) -> &ContextActionMap {
        &self.context_map
    }

    pub fn get_context_map_mut(&mut self) -> &mut ContextActionMap {
        &mut self.context_map
    }

    pub fn rebind(&mut self, combo: KeyCombo, action: InputAction) {
        self.context_map.get_active_map_mut().bind(combo, action);
    }

    // === Backwards Compatibility ===

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
}
