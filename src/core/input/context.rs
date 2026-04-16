// Input Context and Categories - Coral Engine
// Context modes for different editing states

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum InputContext {
    #[default]
    ViewMode, // Free camera navigation
    ObjectMode, // Object selection/transform
    EditMode,   // Vertex/edge/face editing
    PaintMode,  // Texture painting
}

impl InputContext {
    pub fn all() -> &'static [InputContext] {
        &[
            InputContext::ViewMode,
            InputContext::ObjectMode,
            InputContext::EditMode,
            InputContext::PaintMode,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            InputContext::ViewMode => "View",
            InputContext::ObjectMode => "Object",
            InputContext::EditMode => "Edit",
            InputContext::PaintMode => "Paint",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActionCategory {
    Navigation,
    Selection,
    Transform,
    View,
    Edit,
    File,
    UI,
    Game,
}

impl ActionCategory {
    pub fn all() -> &'static [ActionCategory] {
        &[
            ActionCategory::Navigation,
            ActionCategory::Selection,
            ActionCategory::Transform,
            ActionCategory::View,
            ActionCategory::Edit,
            ActionCategory::File,
            ActionCategory::UI,
            ActionCategory::Game,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ActionCategory::Navigation => "Navigation",
            ActionCategory::Selection => "Selection",
            ActionCategory::Transform => "Transform",
            ActionCategory::View => "View",
            ActionCategory::Edit => "Edit",
            ActionCategory::File => "File",
            ActionCategory::UI => "UI",
            ActionCategory::Game => "Game",
        }
    }
}
