// Action State - Coral Engine
// Tracks which actions are active, just pressed, or just released

use super::actions::InputAction;
use std::collections::HashSet;

#[derive(Clone, Debug, Default)]
pub struct ActionState {
    active: HashSet<InputAction>,
    just_pressed: HashSet<InputAction>,
    just_released: HashSet<InputAction>,
}

impl ActionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn press(&mut self, action: InputAction) {
        if self.active.insert(action) {
            self.just_pressed.insert(action);
        }
    }

    pub fn release(&mut self, action: InputAction) {
        if self.active.remove(&action) {
            self.just_released.insert(action);
        }
    }

    pub fn is_active(&self, action: InputAction) -> bool {
        self.active.contains(&action)
    }

    pub fn is_just_pressed(&self, action: InputAction) -> bool {
        self.just_pressed.contains(&action)
    }

    pub fn is_just_released(&self, action: InputAction) -> bool {
        self.just_released.contains(&action)
    }

    pub fn active_actions(&self) -> impl Iterator<Item = &InputAction> {
        self.active.iter()
    }

    pub fn clear_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}
