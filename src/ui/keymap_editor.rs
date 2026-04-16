// Keymap Editor Panel - Coral Engine
// Simple list-based UI for viewing keybindings

use crate::core::input::{ContextActionMap, InputAction, InputContext};

#[derive(Clone, Debug, Default)]
pub struct KeymapEditorState {
    pub selected_context: InputContext,
    pub selected_action: Option<InputAction>,
}

impl KeymapEditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, ctx: &egui::Context, action_map: &ContextActionMap) {
        egui::Window::new("Keymap Editor")
            .resizable(true)
            .default_size([500.0, 350.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Context:");
                    egui::ComboBox::from_id_salt("keymap_context")
                        .selected_text(self.selected_context.name())
                        .show_ui(ui, |ui| {
                            for ctx in InputContext::all() {
                                ui.selectable_value(&mut self.selected_context, *ctx, ctx.name());
                            }
                        });
                });

                ui.separator();
                ui.label("Key Bindings:");
                ui.separator();

                let map = action_map.get_map(self.selected_context);
                let bindings: Vec<_> = map.all_bindings().collect();

                egui::ScrollArea::vertical()
                    .stick_to_right(true)
                    .show(ui, |ui| {
                        for (combo, action) in bindings {
                            ui.horizontal(|ui| {
                                ui.label(format!("{:?}", combo.key));
                                ui.label(" | ");
                                ui.label(combo.modifiers.to_string());
                                ui.label(" | ");
                                ui.label(action.name());
                            });
                        }
                    });

                ui.separator();
                ui.label(egui::RichText::new("Ctrl+K to toggle").small());
            });
    }
}
