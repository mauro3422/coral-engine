// Scene Panel - Scene management UI state

#[derive(Clone, Debug, Default)]
pub struct ScenePanelState {
    pub dirty: bool,
    pub scene_name_input: String,
    pub show_save_dialog: bool,
    pub show_load_dialog: bool,
}

impl ScenePanelState {
    pub fn new() -> Self {
        Self {
            dirty: false,
            scene_name_input: String::new(),
            show_save_dialog: false,
            show_load_dialog: false,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}
