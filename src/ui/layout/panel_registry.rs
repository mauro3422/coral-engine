// Panel Registry - Coral Engine
// Trait-based panel content system

use egui::{vec2, Ui};
use std::collections::HashMap;

pub use egui::Vec2 as PanelVec2;

pub trait PanelContentTrait: Send {
    fn title(&self) -> String;
    fn render(&mut self, ui: &mut Ui);
    fn min_size(&self) -> PanelVec2;
    fn supports_floating(&self) -> bool {
        true
    }
    fn on_open(&mut self) {}
    fn on_close(&mut self) {}
}

// Predefined panel IDs
pub const PANEL_OUTLINER: &str = "outliner";
pub const PANEL_PROPERTIES: &str = "properties";
pub const PANEL_OCEAN_CONFIG: &str = "ocean_config";
pub const PANEL_STATS: &str = "stats";
pub const PANEL_CONTROLS: &str = "controls";

pub struct PanelRegistry {
    panels: HashMap<String, Box<dyn PanelContentTrait>>,
}

impl Default for PanelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PanelRegistry {
    pub fn new() -> Self {
        Self {
            panels: HashMap::new(),
        }
    }

    pub fn register<T: PanelContentTrait + 'static>(&mut self, id: &str, panel: T) {
        self.panels.insert(id.to_string(), Box::new(panel));
    }

    pub fn get(&self, id: &str) -> Option<&dyn PanelContentTrait> {
        self.panels.get(id).map(|p| p.as_ref())
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut (dyn PanelContentTrait + 'static)> {
        self.panels.get_mut(id).map(|p| p.as_mut())
    }

    pub fn unregister(&mut self, id: &str) -> bool {
        self.panels.remove(id).is_some()
    }

    pub fn contains(&self, id: &str) -> bool {
        self.panels.contains_key(id)
    }

    pub fn list_panels(&self) -> Vec<&String> {
        self.panels.keys().collect()
    }

    pub fn count(&self) -> usize {
        self.panels.len()
    }
}

// Built-in panel implementations

pub struct OutlinerPanel {
    search_filter: String,
}

impl Default for OutlinerPanel {
    fn default() -> Self {
        Self {
            search_filter: String::new(),
        }
    }
}

impl PanelContentTrait for OutlinerPanel {
    fn title(&self) -> String {
        "Outliner".to_string()
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("Scene Objects");
        ui.separator();

        ui.label("Search:");
        ui.text_edit_singleline(&mut self.search_filter);

        ui.separator();
        ui.label("No objects in scene");

        ui.separator();
        if ui.button("Add Object").clicked() {
            // TODO: implement
        }
    }

    fn min_size(&self) -> PanelVec2 {
        vec2(180.0, 200.0)
    }
    fn supports_floating(&self) -> bool {
        true
    }
}

pub struct PropertiesPanel {
    selected_object: Option<String>,
}

impl Default for PropertiesPanel {
    fn default() -> Self {
        Self {
            selected_object: None,
        }
    }
}

impl PanelContentTrait for PropertiesPanel {
    fn title(&self) -> String {
        "Properties".to_string()
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("Object Properties");
        ui.separator();

        if let Some(obj) = &self.selected_object {
            ui.label(format!("Selected: {}", obj));
            ui.separator();
            ui.label("Transform");
            ui.label("Position: 0, 0, 0");
            ui.label("Rotation: 0, 0, 0");
            ui.label("Scale: 1, 1, 1");
        } else {
            ui.label("No object selected");
            ui.label("Click an object in the outliner");
        }
    }

    fn min_size(&self) -> PanelVec2 {
        vec2(250.0, 300.0)
    }
    fn supports_floating(&self) -> bool {
        true
    }
}

pub struct OceanConfigPanel {
    pub voxel_size: f32,
    pub block_size: f32,
    pub wave_height: f32,
    pub wave_speed: f32,
}

impl Default for OceanConfigPanel {
    fn default() -> Self {
        Self {
            voxel_size: 0.5,
            block_size: 8.0,
            wave_height: 0.5,
            wave_speed: 1.0,
        }
    }
}

impl PanelContentTrait for OceanConfigPanel {
    fn title(&self) -> String {
        "Ocean Config".to_string()
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("Ocean Settings");
        ui.separator();

        ui.label("Voxel Size:");
        ui.add(egui::Slider::new(&mut self.voxel_size, 0.1..=2.0));

        ui.label("Block Size:");
        ui.add(egui::Slider::new(&mut self.block_size, 1.0..=16.0));

        ui.separator();
        ui.label("Wave Settings");

        ui.label("Height:");
        ui.add(egui::Slider::new(&mut self.wave_height, 0.0..=2.0));

        ui.label("Speed:");
        ui.add(egui::Slider::new(&mut self.wave_speed, 0.0..=5.0));
    }

    fn min_size(&self) -> PanelVec2 {
        vec2(220.0, 350.0)
    }
    fn supports_floating(&self) -> bool {
        true
    }
}

pub struct StatsPanel {
    fps: f64,
    vertices: usize,
    triangles: usize,
    draw_calls: usize,
}

impl Default for StatsPanel {
    fn default() -> Self {
        Self {
            fps: 60.0,
            vertices: 0,
            triangles: 0,
            draw_calls: 0,
        }
    }
}

impl PanelContentTrait for StatsPanel {
    fn title(&self) -> String {
        "Statistics".to_string()
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("Performance");
        ui.separator();

        ui.label(format!("FPS: {:.0}", self.fps));

        ui.separator();
        ui.label("Geometry");
        ui.label(format!("Vertices: {}", self.vertices));
        ui.label(format!("Triangles: {}", self.triangles));
        ui.label(format!("Draw Calls: {}", self.draw_calls));
    }

    fn min_size(&self) -> PanelVec2 {
        vec2(150.0, 150.0)
    }
    fn supports_floating(&self) -> bool {
        true
    }
}

pub struct ControlsPanel;

impl PanelContentTrait for ControlsPanel {
    fn title(&self) -> String {
        "Controls".to_string()
    }

    fn render(&mut self, ui: &mut Ui) {
        ui.heading("Controls");
        ui.separator();

        ui.label("WASD - Move");
        ui.label("Space/Shift - Up/Down");
        ui.label("Q/E - Speed");
        ui.label("L-Click - Look");
        ui.label("Scroll - Zoom");
        ui.label("Tab - Mode");
        ui.label("ESC - Exit");
    }

    fn min_size(&self) -> PanelVec2 {
        vec2(180.0, 180.0)
    }
    fn supports_floating(&self) -> bool {
        false
    }
}

// Helper to create registry with all built-in panels
pub fn create_default_registry() -> PanelRegistry {
    let mut registry = PanelRegistry::new();

    registry.register(PANEL_OUTLINER, OutlinerPanel::default());
    registry.register(PANEL_PROPERTIES, PropertiesPanel::default());
    registry.register(PANEL_OCEAN_CONFIG, OceanConfigPanel::default());
    registry.register(PANEL_STATS, StatsPanel::default());
    registry.register(PANEL_CONTROLS, ControlsPanel);

    registry
}
