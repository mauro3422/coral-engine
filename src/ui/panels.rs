// Editor Panels - Coral Engine
// Compact layout: Right side panel (Outliner + Properties stacked), Controls window

use egui::Slider;
use crate::core::scene::{ObjectId, Scene, SceneProperty};
use crate::ocean::OceanWorld;
use crate::core::config::RenderConfig;
use crate::ui::editor::EditorState;

#[derive(Clone, Debug, Default)]
pub struct EditorAction {
    pub select_object: Option<ObjectId>,
    pub deselect_all: bool,
    pub modified_property: Option<(ObjectId, usize, SceneProperty)>,
    pub ocean_rebuild_needed: bool,
}

/// Right side panel - Outliner + Properties stacked, doesn't block viewport
pub fn render_right_panel(
    ctx: &egui::Context,
    scene: &Scene,
    editor: &mut EditorState,
    ocean_world: &mut OceanWorld,
    render_config: &mut RenderConfig,
) -> EditorAction {
    let mut action = EditorAction::default();

    egui::SidePanel::right("right_panel")
        .default_width(280.0)
        .resizable(true)
        .show(ctx, |ui| {
            // === OUTLINER ===
            ui.heading("📋 Scene");
            ui.separator();
            for obj in &scene.objects {
                let selected = editor.selection.is_selected(obj.id);
                if ui.selectable_label(selected, format!("{} {}", obj.obj_type.icon(), obj.name)).clicked() {
                    action.select_object = Some(obj.id);
                }
            }
            ui.separator();

            // === PROPERTIES (only if object selected) ===
            if let Some(id) = editor.selection.selected_id {
                if let Some(obj) = scene.get_object(id) {
                    ui.horizontal(|ui| {
                        ui.label(obj.obj_type.icon());
                        ui.strong(&obj.name);
                    });

                    // Transform info (compact, read-only)
                    ui.horizontal(|ui| {
                        ui.small("Pos:");
                        ui.small(format!("{:.1}", obj.position[1])); // Just Y
                    });
                    ui.horizontal(|ui| {
                        ui.small("Scale:");
                        ui.small(format!("{:.1}x{:.1}x{:.1}", obj.scale[0], obj.scale[1], obj.scale[2]));
                    });

                    // Ocean properties - COMPACT sliders
                    ui.separator();
                    ui.small("🌊 Voxel Size");
                    let mut vs = ocean_world.config.voxel_size;
                    if ui.add(Slider::new(&mut vs, 0.1..=2.0).step_by(0.05)).changed() {
                        ocean_world.config.voxel_size = vs;
                        ocean_world.config.recalculate_block_size();
                        ocean_world.generate();
                        action.ocean_rebuild_needed = true;
                    }

                    ui.small("🧱 Block Size (m)");
                    let mut bs = ocean_world.config.block_world_size;
                    if ui.add(Slider::new(&mut bs, 1.0..=16.0).step_by(0.5)).changed() {
                        ocean_world.config.block_world_size = bs;
                        ocean_world.config.recalculate_block_size();
                        ocean_world.generate();
                        action.ocean_rebuild_needed = true;
                    }

                    ui.small("🌊 Layers");
                    let mut wl = ocean_world.config.water_layers;
                    if ui.add(Slider::new(&mut wl, 1u32..=4u32)).changed() {
                        ocean_world.config.water_layers = wl;
                        ocean_world.generate();
                        action.ocean_rebuild_needed = true;
                    }

                    ui.small("📏 Wave Height");
                    ui.add(Slider::new(&mut ocean_world.config.wave_height, 0.0..=2.0).step_by(0.05));

                    ui.small("⚡ Wave Speed");
                    ui.add(Slider::new(&mut ocean_world.config.wave_speed, 0.0..=5.0).step_by(0.1));

                    ui.checkbox(&mut ocean_world.config.enable_animation, "▶️ Animation");

                    // Render toggles
                    ui.separator();
                    ui.small("🎨 Layers");
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut render_config.show_water, "🌊");
                        ui.checkbox(&mut render_config.show_grid, "📐");
                        ui.checkbox(&mut render_config.show_axes, "🎯");
                    });
                }
            } else {
                ui.small("Click an object to edit");
            }
        });

    action
}

/// Controls Window - Always visible, compact, corner
pub fn render_controls_panel(ctx: &egui::Context) {
    egui::Window::new("🎮 Controls")
        .anchor(egui::Align2::RIGHT_BOTTOM, (-10.0, -35.0))
        .default_width(220.0)
        .resizable(false)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| { ui.label("🔹"); ui.label(egui::RichText::new("WASD").monospace()); ui.small("- Move"); });
            ui.horizontal(|ui| { ui.label("🔹"); ui.label(egui::RichText::new("Space/Shift").monospace()); ui.small("- Up/Down"); });
            ui.horizontal(|ui| { ui.label("🔹"); ui.label(egui::RichText::new("Q/E").monospace()); ui.small("- Speed"); });
            ui.horizontal(|ui| { ui.label("🔹"); ui.label(egui::RichText::new("L-Click").monospace()); ui.small("- Look"); });
            ui.horizontal(|ui| { ui.label("🔹"); ui.label(egui::RichText::new("Scroll").monospace()); ui.small("- Zoom"); });
            ui.horizontal(|ui| { ui.label("🔹"); ui.label(egui::RichText::new("ESC").monospace()); ui.small("- Exit"); });
        });
}

/// Status bar - Thin bottom info
pub fn render_status_bar(
    ctx: &egui::Context,
    scene: &Scene,
    editor: &EditorState,
    fps: f64,
    active_voxels: usize,
) {
    egui::TopBottomPanel::bottom("statusbar")
        .default_height(22.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.small("🪸 Coral v0.4.0");
                ui.separator();
                ui.small(format!("Objects: {}", scene.objects.len()));
                ui.separator();
                ui.small(format!("Voxels: {}", active_voxels));
                ui.separator();
                ui.small(format!("FPS: {:.0}", fps));
                if let Some(id) = editor.selection.selected_id {
                    if let Some(obj) = scene.get_object(id) {
                        ui.separator();
                        ui.small(format!("Selected: {} {}", obj.obj_type.icon(), obj.name));
                    }
                }
            });
        });
}
