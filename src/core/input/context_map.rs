// Context Action Map - Coral Engine
// Manages ActionMaps for each InputContext with Blender-style defaults

use super::action_map::ActionMap;
use super::actions::InputAction;
use super::context::InputContext;
use super::key_modifiers::KeyCombo;
use std::collections::HashMap;
use winit::keyboard::KeyCode;

#[derive(Clone, Debug)]
pub struct ContextActionMap {
    maps: HashMap<InputContext, ActionMap>,
    active: InputContext,
}

impl Default for ContextActionMap {
    fn default() -> Self {
        Self::blender_style()
    }
}

impl ContextActionMap {
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        for context in InputContext::all() {
            maps.insert(*context, ActionMap::new());
        }
        Self {
            maps,
            active: InputContext::ViewMode,
        }
    }

    pub fn blender_style() -> Self {
        let mut this = Self::new();

        // === VIEW MODE ===
        let view_map = this.maps.get_mut(&InputContext::ViewMode).unwrap();

        // Navigation - WASD + Space/Shift
        view_map.bind(KeyCombo::from_key(KeyCode::KeyW), InputAction::MoveForward);
        view_map.bind(KeyCombo::from_key(KeyCode::KeyS), InputAction::MoveBackward);
        view_map.bind(KeyCombo::from_key(KeyCode::KeyA), InputAction::MoveLeft);
        view_map.bind(KeyCombo::from_key(KeyCode::KeyD), InputAction::MoveRight);
        view_map.bind(KeyCombo::from_key(KeyCode::Space), InputAction::MoveUp);
        view_map.bind(
            KeyCombo::from_key(KeyCode::ShiftLeft),
            InputAction::MoveDown,
        );

        // Speed modifiers
        view_map.bind(KeyCombo::from_key(KeyCode::KeyQ), InputAction::SpeedBoost);
        view_map.bind(KeyCombo::from_key(KeyCode::KeyE), InputAction::SpeedSlow);

        // View shortcuts (Numpad - Blender style)
        view_map.bind(KeyCombo::from_key(KeyCode::Numpad7), InputAction::ViewTop);
        view_map.bind(KeyCombo::from_key(KeyCode::Numpad1), InputAction::ViewFront);
        view_map.bind(KeyCombo::from_key(KeyCode::Numpad3), InputAction::ViewRight);
        view_map.bind(KeyCombo::from_key(KeyCode::Numpad9), InputAction::ViewBack);
        view_map.bind(
            KeyCombo::from_key(KeyCode::Numpad5),
            InputAction::ViewPerspective,
        );

        // Context switching
        view_map.bind(
            KeyCombo::from_key(KeyCode::Tab),
            InputAction::SwitchToObjectMode,
        );

        // File operations
        view_map.bind(KeyCombo::with_ctrl(KeyCode::KeyS), InputAction::SaveScene);
        view_map.bind(KeyCombo::with_ctrl(KeyCode::KeyO), InputAction::OpenScene);
        view_map.bind(KeyCombo::with_ctrl(KeyCode::KeyN), InputAction::NewScene);

        // UI operations
        view_map.bind(KeyCombo::with_ctrl(KeyCode::KeyZ), InputAction::Undo);
        view_map.bind(KeyCombo::with_ctrl(KeyCode::KeyY), InputAction::Redo);

        // === OBJECT MODE ===
        let object_map = this.maps.get_mut(&InputContext::ObjectMode).unwrap();

        // Transform
        object_map.bind(KeyCombo::from_key(KeyCode::KeyG), InputAction::Grab);
        object_map.bind(KeyCombo::from_key(KeyCode::KeyR), InputAction::Rotate);
        object_map.bind(KeyCombo::from_key(KeyCode::KeyS), InputAction::Scale);

        // Selection
        object_map.bind(KeyCombo::from_key(KeyCode::KeyA), InputAction::SelectAll);
        object_map.bind(KeyCombo::from_key(KeyCode::KeyX), InputAction::Delete);

        // Context switching
        object_map.bind(
            KeyCombo::from_key(KeyCode::Tab),
            InputAction::SwitchToEditMode,
        );

        // === EDIT MODE ===
        let edit_map = this.maps.get_mut(&InputContext::EditMode).unwrap();

        // Mode switching
        edit_map.bind(
            KeyCombo::from_key(KeyCode::Tab),
            InputAction::SwitchToObjectMode,
        );
        edit_map.bind(
            KeyCombo::from_key(KeyCode::Digit1),
            InputAction::EditModeVertex,
        );
        edit_map.bind(
            KeyCombo::from_key(KeyCode::Digit2),
            InputAction::EditModeEdge,
        );
        edit_map.bind(
            KeyCombo::from_key(KeyCode::Digit3),
            InputAction::EditModeFace,
        );

        // === PAINT MODE ===
        let paint_map = this.maps.get_mut(&InputContext::PaintMode).unwrap();
        paint_map.bind(
            KeyCombo::from_key(KeyCode::Tab),
            InputAction::SwitchToObjectMode,
        );

        this
    }

    pub fn active_context(&self) -> InputContext {
        self.active
    }

    pub fn set_active_context(&mut self, context: InputContext) {
        self.active = context;
    }

    pub fn get_active_map(&self) -> &ActionMap {
        self.maps.get(&self.active).unwrap()
    }

    pub fn get_active_map_mut(&mut self) -> &mut ActionMap {
        self.maps.get_mut(&self.active).unwrap()
    }

    pub fn get_map(&self, context: InputContext) -> &ActionMap {
        self.maps.get(&context).unwrap()
    }

    pub fn get_map_mut(&mut self, context: InputContext) -> &mut ActionMap {
        self.maps.get_mut(&context).unwrap()
    }
}
