// Editor Panels - Coral Engine
// Compact layout: Right side panel (Outliner + Properties stacked), Controls window

use crate::core::config::RenderConfig;
use crate::core::persist::SceneManager;
use crate::core::scene::{ObjectId, Scene, SceneProperty};
use crate::ocean::{BlockPos, BlockRegistry, BlockTag, OceanWorld};
use crate::ui::block_panel::BlockBrowserState;
use crate::ui::editor::EditorState;
use crate::ui::scene_panel::ScenePanelState;
use egui::Slider;

#[derive(Clone, Debug, Default)]
pub struct EditorAction {
    pub select_object: Option<ObjectId>,
    pub deselect_all: bool,
    pub modified_property: Option<(ObjectId, usize, SceneProperty)>,
    pub ocean_rebuild_needed: bool,
    pub scene_save: Option<String>,
    pub scene_load: Option<String>,
    pub select_block: Option<BlockPos>,
}

impl EditorAction {
    pub fn none() -> Self {
        Self::default()
    }
}

pub fn render_right_panel(
    ctx: &egui::Context,
    scene: &Scene,
    editor: &mut EditorState,
    ocean_world: &mut OceanWorld,
    render_config: &mut RenderConfig,
    scene_manager: Option<&mut SceneManager>,
    scene_panel: &mut ScenePanelState,
    block_registry: &mut BlockRegistry,
    block_browser: &mut BlockBrowserState,
) -> EditorAction {
    let mut action = EditorAction::default();

    egui::SidePanel::right("right_panel")
        .default_width(280.0)
        .resizable(true)
        .show(ctx, |ui| {
            // === SCENE MANAGER ===
            ui.heading("💾 Scene");

            // Scene name input
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut scene_panel.scene_name_input);
            });

            ui.horizontal(|ui| {
                if ui.button("💾 Save").clicked() {
                    if !scene_panel.scene_name_input.is_empty() {
                        action.scene_save = Some(scene_panel.scene_name_input.clone());
                    }
                }
            });

            // Show dirty indicator
            if scene_panel.dirty {
                ui.label(egui::RichText::new("* Unsaved").color(egui::Color32::RED));
            }

            // Scene list
            if let Some(mgr) = scene_manager {
                let scenes = mgr.list_scenes().unwrap_or_default();
                ui.label(format!("Saved: {} scenes", scenes.len()));
                if !scenes.is_empty() {
                    ui.separator();
                    ui.label("Load:");
                    for (_, name) in &scenes {
                        if ui.button(name).clicked() {
                            action.scene_load = Some(name.clone());
                        }
                    }
                }
            }

            ui.separator();

            // === BLOCK BROWSER ===
            ui.heading("📦 Blocks");
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.text_edit_singleline(&mut block_browser.filter_name);
            });
            ui.horizontal(|ui| {
                ui.label("🏷️");
                ui.text_edit_singleline(&mut block_browser.filter_tag);
            });
            ui.checkbox(&mut block_browser.show_hidden, "Show Hidden");

            let blocks: Vec<_> = block_registry.all_positions().iter().copied().collect();
            let visible_count = blocks
                .iter()
                .filter(|pos| {
                    if let Some(meta) = block_registry.get_metadata(**pos) {
                        let visible = meta.hidden && !block_browser.show_hidden;
                        let name_match = block_browser.filter_name.is_empty()
                            || meta
                                .name
                                .to_lowercase()
                                .contains(&block_browser.filter_name.to_lowercase());
                        let tag_match = block_browser.filter_tag.is_empty()
                            || meta.has_tag(&BlockTag::new(&block_browser.filter_tag));
                        !visible && name_match && tag_match
                    } else {
                        false
                    }
                })
                .count();

            ui.label(format!("Showing: {} / {}", visible_count, blocks.len()));

            // Filtered block list
            egui::ScrollArea::vertical().show(ui, |ui| {
                for pos in &blocks {
                    let Some(meta) = block_registry.get_metadata(*pos) else {
                        continue;
                    };
                    if meta.hidden && !block_browser.show_hidden {
                        continue;
                    }
                    if !block_browser.filter_name.is_empty()
                        && !meta
                            .name
                            .to_lowercase()
                            .contains(&block_browser.filter_name.to_lowercase())
                    {
                        continue;
                    }
                    if !block_browser.filter_tag.is_empty()
                        && !meta.has_tag(&BlockTag::new(&block_browser.filter_tag))
                    {
                        continue;
                    }

                    let selected = block_browser.selected_block == Some(*pos);
                    let label = if meta.hidden {
                        format!("🚫 {}", meta.name)
                    } else if meta.locked {
                        format!("🔒 {}", meta.name)
                    } else {
                        format!("📦 {}", meta.name)
                    };

                    if ui.selectable_label(selected, label).clicked() {
                        block_browser.selected_block = Some(*pos);
                        action.select_block = Some(*pos);
                    }
                }
            });

            ui.separator();

            // === OUTLINER ===
            ui.heading("📋 Scene");
            ui.separator();
            for obj in &scene.objects {
                let selected = editor.selection.is_selected(obj.id);
                if ui
                    .selectable_label(selected, format!("{} {}", obj.obj_type.icon(), obj.name))
                    .clicked()
                {
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
                        ui.small(format!(
                            "{:.1}x{:.1}x{:.1}",
                            obj.scale[0], obj.scale[1], obj.scale[2]
                        ));
                    });

                    // Ocean properties - COMPACT sliders
                    ui.separator();
                    ui.small("🌊 Voxel Size");
                    let mut vs = ocean_world.config.voxel_size;
                    if ui
                        .add(Slider::new(&mut vs, 0.1..=2.0).step_by(0.05))
                        .changed()
                    {
                        ocean_world.config.voxel_size = vs;
                        ocean_world.config.recalculate_block_size();
                        ocean_world.generate();
                        action.ocean_rebuild_needed = true;
                    }

                    ui.small("🧱 Block Size (m)");
                    let mut bs = ocean_world.config.block_world_size;
                    if ui
                        .add(Slider::new(&mut bs, 1.0..=16.0).step_by(0.5))
                        .changed()
                    {
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
                    ui.add(
                        Slider::new(&mut ocean_world.config.wave_height, 0.0..=2.0).step_by(0.05),
                    );

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
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("WASD").monospace());
                ui.small("- Move");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("Space/Shift").monospace());
                ui.small("- Up/Down");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("Q/E").monospace());
                ui.small("- Speed");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("L-Click").monospace());
                ui.small("- Look");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("Scroll").monospace());
                ui.small("- Zoom");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("ESC").monospace());
                ui.small("- Exit");
            });
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
