//! Editor State - Central state for the Blender-style interface
//! Manages selection, tools, and panel visibility
//!
//! Note: This module has dead_code because UI features are not fully implemented

use crate::core::scene::{ObjectId, SceneProperty};

/// Currently selected item in the editor
#[derive(Clone, Copy, Debug, Default)]
pub struct EditorSelection {
    pub selected_id: Option<ObjectId>,
    pub hovered_id: Option<ObjectId>,
}

impl EditorSelection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn select(&mut self, id: ObjectId) {
        self.selected_id = Some(id);
    }

    pub fn deselect(&mut self) {
        self.selected_id = None;
    }

    pub fn is_selected(&self, id: ObjectId) -> bool {
        self.selected_id == Some(id)
    }
}

/// Which panels are visible
#[derive(Clone, Debug)]
pub struct EditorPanels {
    pub show_outliner: bool,
    pub show_properties: bool,
    pub show_add_menu: bool,
    pub show_ocean_config: bool,
    pub show_stats: bool,
    pub show_controls: bool,
}

impl EditorPanels {
    pub fn default_editor() -> Self {
        Self {
            show_outliner: true,
            show_properties: true,
            show_add_menu: false,
            show_ocean_config: false,
            show_stats: false,
            show_controls: true,
        }
    }
}

/// Main editor state
#[derive(Debug)]
pub struct EditorState {
    pub selection: EditorSelection,
    pub panels: EditorPanels,
    pub selected_properties: Vec<SceneProperty>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            selection: EditorSelection::new(),
            panels: EditorPanels::default_editor(),
            selected_properties: Vec::new(),
        }
    }

    pub fn select_object(&mut self, id: ObjectId) {
        self.selection.select(id);
    }

    pub fn deselect_all(&mut self) {
        self.selection.deselect();
        self.selected_properties.clear();
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
