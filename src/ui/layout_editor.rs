// Layout Editor Panel - Coral Engine

use crate::ui::layout::LayoutState;
use crate::ui::panel_visibility::PanelName;

#[derive(Clone, Debug, Default)]
pub struct LayoutEditorState {
    pub toggle_requests: Vec<PanelName>,
}

impl LayoutEditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(
        &mut self,
        ctx: &egui::Context,
        layout: &LayoutState,
        panel_vis: &crate::ui::panel_visibility::PanelVisibility,
    ) {
        self.toggle_requests.clear();

        egui::Window::new("Layout Editor")
            .resizable(true)
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("Panel Layout").heading());
                ui.separator();

                ui.label("Toggle Panels:");
                ui.horizontal(|ui| {
                    if ui.button("Outliner").clicked() {
                        self.toggle_requests.push(PanelName::Outliner);
                    }
                    if ui.button("Properties").clicked() {
                        self.toggle_requests.push(PanelName::Properties);
                    }
                    if ui.button("Stats").clicked() {
                        self.toggle_requests.push(PanelName::Stats);
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Controls").clicked() {
                        self.toggle_requests.push(PanelName::Controls);
                    }
                    if ui.button("Ocean Config").clicked() {
                        self.toggle_requests.push(PanelName::OceanConfig);
                    }
                });

                ui.separator();
                ui.label("Layout Info:");
                ui.label(format!("Total panels: {}", layout.panel_count()));

                ui.label("Visible:");
                ui.label(format!("  Outliner: {}", panel_vis.is_visible(PanelName::Outliner)));
                ui.label(format!("  Properties: {}", panel_vis.is_visible(PanelName::Properties)));
                ui.label(format!("  Stats: {}", panel_vis.is_visible(PanelName::Stats)));
                ui.label(format!("  Controls: {}", panel_vis.is_visible(PanelName::Controls)));

                ui.separator();
                ui.label(egui::RichText::new("Hotkeys: K=Keymap, L=Layout, O=Outliner, P=Properties, 1=Stats, 2=Controls").small());
            });
    }

    pub fn take_toggle_requests(&mut self) -> Vec<PanelName> {
        std::mem::take(&mut self.toggle_requests)
    }
}
