#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiLayerKind {
    Panel,
    FloatingWindow,
    Widget,
}

#[derive(Clone, Copy, Debug)]
pub struct UiLayer {
    pub kind: UiLayerKind,
    pub min: (f32, f32),
    pub max: (f32, f32),
}

impl UiLayer {
    pub fn contains(&self, point: (f64, f64), pixels_per_point: f64) -> bool {
        let x = (point.0 / pixels_per_point) as f32;
        let y = (point.1 / pixels_per_point) as f32;
        x >= self.min.0 && x <= self.max.0 && y >= self.min.1 && y <= self.max.1
    }
}

#[derive(Clone, Debug, Default)]
pub struct UiLayerRegistry {
    pub layers: Vec<UiLayer>,
    pub panel_min: (f32, f32),
    pub panel_max: (f32, f32),
    pub panel_set: bool,
}

impl UiLayerRegistry {
    pub fn clear(&mut self) {
        self.layers.clear();
        self.panel_set = false;
    }

    pub fn push(&mut self, layer: UiLayer) {
        self.layers.push(layer);
    }

    pub fn track_widget(&mut self, response: &egui::Response, _name: &str) {
        // Track any response that's enabled - checkbox/radio/button always returns enabled
        if response.enabled() {
            let rect = response.rect;
            self.layers.push(UiLayer {
                kind: UiLayerKind::Widget,
                min: (rect.min.x, rect.min.y),
                max: (rect.max.x, rect.max.y),
            });
        }
    }

    pub fn set_panel_bounds(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        self.panel_min = (min_x, min_y);
        self.panel_max = (max_x, max_y);
        self.panel_set = true;
    }

    pub fn blocks_point(&self, point: (f64, f64), pixels_per_point: f64) -> bool {
        // Check explicit widgets first
        if self
            .layers
            .iter()
            .any(|layer| layer.contains(point, pixels_per_point))
        {
            return true;
        }
        // Check panel bounds if set
        if self.panel_set {
            let x = (point.0 / pixels_per_point) as f32;
            let y = (point.1 / pixels_per_point) as f32;
            return x >= self.panel_min.0
                && x <= self.panel_max.0
                && y >= self.panel_min.1
                && y <= self.panel_max.1;
        }
        false
    }

    pub fn is_click_on_widget(&self, point: (f64, f64), pixels_per_point: f64) -> bool {
        self.layers.iter().any(|layer| {
            if layer.kind == UiLayerKind::Widget {
                layer.contains(point, pixels_per_point)
            } else {
                false
            }
        })
    }

    pub fn count_kind(&self, kind: UiLayerKind) -> usize {
        self.layers
            .iter()
            .filter(|layer| layer.kind == kind)
            .count()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum WorldCaptureState {
    #[default]
    Idle,
    Armed,
    Active,
}

#[derive(Clone, Debug, Default)]
pub struct InputRouter {
    pub ui_layers: UiLayerRegistry,
    pub pending_layers: UiLayerRegistry,
    pub wants_pointer: bool,
    pub wants_keyboard: bool,
    pub pointer_over_ui: bool,
    pub pending_wants_pointer: bool,
    pub pending_wants_keyboard: bool,
    pub pending_pointer_over_ui: bool,
    pub world: WorldCaptureState,
}

impl InputRouter {
    pub fn reset_frame(&mut self) {
        self.pending_layers.clear();
        self.pending_wants_pointer = false;
        self.pending_wants_keyboard = false;
        self.pending_pointer_over_ui = false;
    }

    pub fn set_ui_layers(&mut self, registry: UiLayerRegistry) {
        self.pending_layers = registry;
    }

    pub fn track_layer(&mut self, layer: &UiLayer) {
        self.pending_layers.push(*layer);
    }

    pub fn register_layer(&mut self, kind: UiLayerKind, min: (f32, f32), max: (f32, f32)) {
        self.pending_layers.push(UiLayer { kind, min, max });
    }

    pub fn set_egui_state(
        &mut self,
        wants_pointer: bool,
        wants_keyboard: bool,
        pointer_over_ui: bool,
    ) {
        self.pending_wants_pointer = wants_pointer;
        self.pending_wants_keyboard = wants_keyboard;
        self.pending_pointer_over_ui = pointer_over_ui;
    }

    pub fn commit_frame(&mut self) {
        self.ui_layers = self.pending_layers.clone();
        self.wants_pointer = self.pending_wants_pointer;
        self.wants_keyboard = self.pending_wants_keyboard;
        self.pointer_over_ui = self.pending_pointer_over_ui;
    }

    pub fn blocks_world_pointer(&self, point: (f64, f64), pixels_per_point: f64) -> bool {
        self.wants_pointer
            || self.pointer_over_ui
            || self.ui_layers.blocks_point(point, pixels_per_point)
    }

    pub fn blocks_world_keyboard(&self) -> bool {
        self.wants_keyboard
    }

    pub fn is_world_active(&self) -> bool {
        matches!(self.world, WorldCaptureState::Active)
    }

    pub fn arm_world_capture(&mut self) {
        if matches!(self.world, WorldCaptureState::Idle) {
            self.world = WorldCaptureState::Armed;
        }
    }

    pub fn activate_world_capture(&mut self) {
        self.world = WorldCaptureState::Active;
    }

    pub fn release_world_capture(&mut self) {
        self.world = WorldCaptureState::Idle;
    }

    pub fn can_arm_world_capture(&self, point: (f64, f64), pixels_per_point: f64) -> bool {
        !self.blocks_world_pointer(point, pixels_per_point)
    }

    pub fn layer_summary(&self) -> String {
        let panels = self.ui_layers.count_kind(UiLayerKind::Panel);
        let floating = self.ui_layers.count_kind(UiLayerKind::FloatingWindow);
        let widgets = self.ui_layers.count_kind(UiLayerKind::Widget);
        let world = match self.world {
            WorldCaptureState::Idle => "idle",
            WorldCaptureState::Armed => "armed",
            WorldCaptureState::Active => "active",
        };

        format!(
            "UI layers: panels {} / floating {} / widgets {} | world {}",
            panels, floating, widgets, world
        )
    }
}
