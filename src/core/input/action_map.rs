// Action Map - Coral Engine
// Maps KeyCombo to InputAction within a single context

use super::actions::InputAction;
use super::key_modifiers::KeyCombo;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ActionMap {
    bindings: HashMap<KeyCombo, InputAction>,
    inverse: HashMap<InputAction, KeyCombo>,
}

impl ActionMap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            inverse: HashMap::new(),
        }
    }

    pub fn bind(&mut self, combo: KeyCombo, action: InputAction) {
        self.bindings.insert(combo, action);
        self.inverse.insert(action, combo);
    }

    pub fn unbind(&mut self, combo: &KeyCombo) {
        if let Some(action) = self.bindings.remove(combo) {
            self.inverse.remove(&action);
        }
    }

    pub fn unbind_action(&mut self, action: InputAction) {
        if let Some(combo) = self.inverse.remove(&action) {
            self.bindings.remove(&combo);
        }
    }

    pub fn action_for(&self, combo: &KeyCombo) -> Option<InputAction> {
        self.bindings.get(combo).copied()
    }

    pub fn combo_for(&self, action: InputAction) -> Option<KeyCombo> {
        self.inverse.get(&action).copied()
    }

    pub fn is_bound(&self, combo: &KeyCombo) -> bool {
        self.bindings.contains_key(combo)
    }

    pub fn all_bindings(&self) -> impl Iterator<Item = (&KeyCombo, &InputAction)> {
        self.bindings.iter()
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
        self.inverse.clear();
    }
}
