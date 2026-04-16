// Input Actions - Coral Engine
// All possible actions the engine can respond to

use super::context::{ActionCategory, InputContext};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputAction {
    // Special
    None,

    // Navigation
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    CameraRotate,
    CameraZoom,
    SpeedBoost,
    SpeedSlow,

    // Selection
    Select,
    SelectAll,
    Deselect,
    InvertSelection,

    // Transform
    Grab,
    Rotate,
    Scale,

    // View
    ViewTop,
    ViewBottom,
    ViewFront,
    ViewBack,
    ViewLeft,
    ViewRight,
    ViewPerspective,
    ViewOrthographic,
    ToggleOrbit,

    // Edit modes
    SwitchToViewMode,
    SwitchToObjectMode,
    SwitchToEditMode,
    SwitchToPaintMode,
    EditModeVertex,
    EditModeEdge,
    EditModeFace,

    // File
    NewScene,
    OpenScene,
    SaveScene,
    Export,

    // UI
    ToggleUI,
    TogglePanel,
    Undo,
    Redo,
    Delete,

    // Game
    Pause,
    Play,
    Interact,
    MouseLook,
    MouseRotate,
    MousePan,
    MouseZoom,
    ContextMenu,
}

impl InputAction {
    pub fn all() -> &'static [InputAction] {
        &[
            InputAction::MoveForward,
            InputAction::MoveBackward,
            InputAction::MoveLeft,
            InputAction::MoveRight,
            InputAction::MoveUp,
            InputAction::MoveDown,
            InputAction::CameraRotate,
            InputAction::CameraZoom,
            InputAction::SpeedBoost,
            InputAction::SpeedSlow,
            InputAction::Select,
            InputAction::SelectAll,
            InputAction::Deselect,
            InputAction::InvertSelection,
            InputAction::Grab,
            InputAction::Rotate,
            InputAction::Scale,
            InputAction::ViewTop,
            InputAction::ViewBottom,
            InputAction::ViewFront,
            InputAction::ViewBack,
            InputAction::ViewLeft,
            InputAction::ViewRight,
            InputAction::ViewPerspective,
            InputAction::ViewOrthographic,
            InputAction::ToggleOrbit,
            InputAction::SwitchToViewMode,
            InputAction::SwitchToObjectMode,
            InputAction::SwitchToEditMode,
            InputAction::SwitchToPaintMode,
            InputAction::EditModeVertex,
            InputAction::EditModeEdge,
            InputAction::EditModeFace,
            InputAction::NewScene,
            InputAction::OpenScene,
            InputAction::SaveScene,
            InputAction::Export,
            InputAction::ToggleUI,
            InputAction::TogglePanel,
            InputAction::Undo,
            InputAction::Redo,
            InputAction::Delete,
            InputAction::Pause,
            InputAction::Play,
            InputAction::Interact,
            InputAction::MouseLook,
            InputAction::MouseRotate,
            InputAction::MousePan,
            InputAction::MouseZoom,
            InputAction::ContextMenu,
        ]
    }

    pub fn category(&self) -> ActionCategory {
        match self {
            InputAction::MoveForward
            | InputAction::MoveBackward
            | InputAction::MoveLeft
            | InputAction::MoveRight
            | InputAction::MoveUp
            | InputAction::MoveDown
            | InputAction::CameraRotate
            | InputAction::CameraZoom
            | InputAction::SpeedBoost
            | InputAction::SpeedSlow => ActionCategory::Navigation,

            InputAction::Select
            | InputAction::SelectAll
            | InputAction::Deselect
            | InputAction::InvertSelection => ActionCategory::Selection,

            InputAction::Grab | InputAction::Rotate | InputAction::Scale => {
                ActionCategory::Transform
            }

            InputAction::ViewTop
            | InputAction::ViewBottom
            | InputAction::ViewFront
            | InputAction::ViewBack
            | InputAction::ViewLeft
            | InputAction::ViewRight
            | InputAction::ViewPerspective
            | InputAction::ViewOrthographic
            | InputAction::ToggleOrbit => ActionCategory::View,

            InputAction::SwitchToViewMode
            | InputAction::SwitchToObjectMode
            | InputAction::SwitchToEditMode
            | InputAction::SwitchToPaintMode
            | InputAction::EditModeVertex
            | InputAction::EditModeEdge
            | InputAction::EditModeFace => ActionCategory::Edit,

            InputAction::NewScene
            | InputAction::OpenScene
            | InputAction::SaveScene
            | InputAction::Export => ActionCategory::File,

            InputAction::ToggleUI
            | InputAction::TogglePanel
            | InputAction::Undo
            | InputAction::Redo
            | InputAction::Delete => ActionCategory::UI,

            _ => ActionCategory::Game,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            InputAction::MoveForward => "Move Forward",
            InputAction::MoveBackward => "Move Backward",
            InputAction::MoveLeft => "Move Left",
            InputAction::MoveRight => "Move Right",
            InputAction::MoveUp => "Move Up",
            InputAction::MoveDown => "Move Down",
            InputAction::CameraRotate => "Camera Rotate",
            InputAction::CameraZoom => "Camera Zoom",
            InputAction::SpeedBoost => "Speed Boost",
            InputAction::SpeedSlow => "Speed Slow",
            InputAction::Select => "Select",
            InputAction::SelectAll => "Select All",
            InputAction::Deselect => "Deselect",
            InputAction::InvertSelection => "Invert Selection",
            InputAction::Grab => "Grab",
            InputAction::Rotate => "Rotate",
            InputAction::Scale => "Scale",
            InputAction::ViewTop => "View Top",
            InputAction::ViewBottom => "View Bottom",
            InputAction::ViewFront => "View Front",
            InputAction::ViewBack => "View Back",
            InputAction::ViewLeft => "View Left",
            InputAction::ViewRight => "View Right",
            InputAction::ViewPerspective => "View Perspective",
            InputAction::ViewOrthographic => "View Orthographic",
            InputAction::ToggleOrbit => "Toggle Orbit",
            InputAction::SwitchToViewMode => "Switch to View Mode",
            InputAction::SwitchToObjectMode => "Switch to Object Mode",
            InputAction::SwitchToEditMode => "Switch to Edit Mode",
            InputAction::SwitchToPaintMode => "Switch to Paint Mode",
            InputAction::EditModeVertex => "Vertex Select",
            InputAction::EditModeEdge => "Edge Select",
            InputAction::EditModeFace => "Face Select",
            InputAction::NewScene => "New Scene",
            InputAction::OpenScene => "Open Scene",
            InputAction::SaveScene => "Save Scene",
            InputAction::Export => "Export",
            InputAction::ToggleUI => "Toggle UI",
            InputAction::TogglePanel => "Toggle Panel",
            InputAction::Undo => "Undo",
            InputAction::Redo => "Redo",
            InputAction::Delete => "Delete",
            InputAction::Pause => "Pause",
            InputAction::Play => "Play",
            InputAction::Interact => "Interact",
            InputAction::MouseLook => "Mouse Look",
            InputAction::MouseRotate => "Mouse Rotate",
            InputAction::MousePan => "Mouse Pan",
            InputAction::MouseZoom => "Mouse Zoom",
            InputAction::ContextMenu => "Context Menu",
            InputAction::None => "None",
        }
    }

    pub fn default_context(&self) -> InputContext {
        match self {
            InputAction::SwitchToObjectMode => InputContext::ViewMode,
            InputAction::SwitchToEditMode => InputContext::ObjectMode,
            InputAction::SwitchToPaintMode => InputContext::EditMode,
            InputAction::SwitchToViewMode => InputContext::ObjectMode,
            InputAction::Grab
            | InputAction::Rotate
            | InputAction::Scale
            | InputAction::SelectAll
            | InputAction::EditModeVertex
            | InputAction::EditModeEdge
            | InputAction::EditModeFace => InputContext::ObjectMode,
            _ => InputContext::ViewMode,
        }
    }
}
