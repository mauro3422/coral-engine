// Panel Visibility - Coral Engine
// Simple panel visibility toggle system

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelName {
    Outliner,
    Properties,
    Stats,
    Controls,
    OceanConfig,
    KeymapEditor,
    LayoutEditor,
}

impl PanelName {
    pub fn all() -> &'static [PanelName] {
        &[
            Self::Outliner,
            Self::Properties,
            Self::Stats,
            Self::Controls,
            Self::OceanConfig,
            Self::KeymapEditor,
            Self::LayoutEditor,
        ]
    }
}

#[derive(Clone, Debug)]
pub struct PanelVisibility {
    outliner: bool,
    properties: bool,
    stats: bool,
    controls: bool,
    ocean_config: bool,
    keymap_editor: bool,
    layout_editor: bool,
}

impl PanelVisibility {
    pub fn new() -> Self {
        Self {
            outliner: true,
            properties: true,
            stats: true,
            controls: true,
            ocean_config: false,
            keymap_editor: false,
            layout_editor: false,
        }
    }

    pub fn is_visible(&self, panel: PanelName) -> bool {
        match panel {
            PanelName::Outliner => self.outliner,
            PanelName::Properties => self.properties,
            PanelName::Stats => self.stats,
            PanelName::Controls => self.controls,
            PanelName::OceanConfig => self.ocean_config,
            PanelName::KeymapEditor => self.keymap_editor,
            PanelName::LayoutEditor => self.layout_editor,
        }
    }

    pub fn set_visible(&mut self, panel: PanelName, visible: bool) {
        match panel {
            PanelName::Outliner => self.outliner = visible,
            PanelName::Properties => self.properties = visible,
            PanelName::Stats => self.stats = visible,
            PanelName::Controls => self.controls = visible,
            PanelName::OceanConfig => self.ocean_config = visible,
            PanelName::KeymapEditor => self.keymap_editor = visible,
            PanelName::LayoutEditor => self.layout_editor = visible,
        }
    }

    pub fn toggle(&mut self, panel: PanelName) {
        let current = self.is_visible(panel);
        self.set_visible(panel, !current);
    }

    pub fn show_all(&mut self) {
        self.outliner = true;
        self.properties = true;
        self.stats = true;
        self.controls = true;
        self.ocean_config = true;
    }

    pub fn hide_all(&mut self) {
        self.outliner = false;
        self.properties = false;
        self.stats = false;
        self.controls = false;
        self.ocean_config = false;
    }
}

impl Default for PanelVisibility {
    fn default() -> Self {
        Self::new()
    }
}
