// Viewport API - Robust UI controls for ocean simulation
// Provides configuration panels, statistics, and runtime controls

use crate::core::cartesian::{Axis3D, CoordinateSystem3D};
use crate::ocean::OceanConfig;
use crate::core::config::RenderConfig;

/// Viewport state - tracks all UI configuration
pub struct ViewportState {
    pub ocean_config: OceanConfig,
    pub render_config: RenderConfig,
    pub show_config_panel: bool,
    pub show_stats_panel: bool,
    pub paused: bool,
}

impl ViewportState {
    pub fn new() -> Self {
        Self {
            ocean_config: OceanConfig::default(),
            render_config: RenderConfig::default(),
            show_config_panel: true,
            show_stats_panel: true,
            paused: false,
        }
    }

    pub fn default() -> Self {
        Self::new()
    }
}

fn axis_label(axis: Axis3D) -> &'static str {
    match axis {
        Axis3D::X => "X",
        Axis3D::Y => "Y",
        Axis3D::Z => "Z",
    }
}

fn axis_color(axis: Axis3D) -> egui::Color32 {
    match axis {
        Axis3D::X => egui::Color32::from_rgb(242, 80, 80),
        Axis3D::Y => egui::Color32::from_rgb(80, 220, 110),
        Axis3D::Z => egui::Color32::from_rgb(80, 130, 255),
    }
}

fn render_axis_gizmo(ui: &mut egui::Ui, system: CoordinateSystem3D) {
    let size = egui::vec2(128.0, 96.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let radius = 28.0;

    let x_end = center + egui::vec2(radius, 0.0);
    let y_end = center + egui::vec2(0.0, -radius);
    let z_end = center + egui::vec2(-radius * 0.7, radius * 0.55);

    painter.line_segment(
        [center, x_end],
        egui::Stroke::new(2.0, axis_color(Axis3D::X)),
    );
    painter.line_segment(
        [center, y_end],
        egui::Stroke::new(2.0, axis_color(Axis3D::Y)),
    );
    painter.line_segment(
        [center, z_end],
        egui::Stroke::new(2.0, axis_color(Axis3D::Z)),
    );

    painter.circle_filled(center, 3.5, egui::Color32::WHITE);
    painter.text(
        x_end + egui::vec2(4.0, -2.0),
        egui::Align2::LEFT_CENTER,
        axis_label(Axis3D::X),
        egui::FontId::proportional(13.0),
        axis_color(Axis3D::X),
    );
    painter.text(
        y_end + egui::vec2(4.0, 0.0),
        egui::Align2::LEFT_CENTER,
        axis_label(Axis3D::Y),
        egui::FontId::proportional(13.0),
        axis_color(Axis3D::Y),
    );
    painter.text(
        z_end + egui::vec2(4.0, 0.0),
        egui::Align2::LEFT_CENTER,
        axis_label(Axis3D::Z),
        egui::FontId::proportional(13.0),
        axis_color(Axis3D::Z),
    );

    ui.label(format!(
        "System: up {} / forward {} / right {}",
        axis_label(system.up_axis),
        axis_label(system.forward_axis),
        axis_label(system.right_axis()),
    ));
}

/// Ocean configuration panel with live controls
pub fn render_ocean_config_panel(
    egui_ctx: &egui::Context,
    viewport: &mut ViewportState,
) -> crate::core::ui_router::UiLayerRegistry {
    let mut registry = crate::core::ui_router::UiLayerRegistry::default();

    if !viewport.show_config_panel {
        return registry;
    }

    egui::Window::new("🌊 Ocean Config")
        .anchor(egui::Align2::LEFT_TOP, (10.0, 10.0))
        .default_size([300.0, 520.0])
        .resizable(true)
        .show(egui_ctx, |ui| {
            ui.heading("Ocean Configuration");
            ui.separator();

            // Voxel size control - changing this auto-recalculates block voxels
            ui.label("📐 Voxel Size (meters per cube):");
            if ui.add(egui::Slider::new(&mut viewport.ocean_config.voxel_size, 0.1..=2.0)
                .step_by(0.05))
                .changed()
            {
                // Block size auto-recalculated during config sync
                println!("[UI] Voxel size -> {:.2}m", viewport.ocean_config.voxel_size);
            }
            ui.label(format!("  Each cube = {:.2}m x {:.2}m x {:.2}m",
                viewport.ocean_config.voxel_size,
                viewport.ocean_config.voxel_size,
                viewport.ocean_config.voxel_size));

            ui.separator();

            // Block physical size control
            ui.label("🧱 Block Physical Size (meters):");
            if ui.add(egui::Slider::new(&mut viewport.ocean_config.block_world_size, 1.0..=16.0)
                .step_by(0.5))
                .changed()
            {
                println!("[UI] Block size -> {:.1}m", viewport.ocean_config.block_world_size);
            }
            // Show calculated voxel count
            let calculated_voxels = (viewport.ocean_config.block_world_size / viewport.ocean_config.voxel_size).round() as i32;
            ui.label(format!("  Block = {:.1}m / {:.2}m = {} voxels/axis",
                viewport.ocean_config.block_world_size,
                viewport.ocean_config.voxel_size,
                calculated_voxels));

            ui.separator();

            // Water layers control
            ui.label("🌊 Water Surface Layers:");
            let mut layers = viewport.ocean_config.water_layers as i32;
            if ui.add(egui::Slider::new(&mut layers, 1..=4)).changed() {
                viewport.ocean_config.water_layers = layers as u32;
            }
            ui.label(format!("  {} layer(s) - only surface water", layers));

            ui.separator();

            // Wave height control
            ui.label("📏 Wave Height (amplitude):");
            if ui.add(egui::Slider::new(&mut viewport.ocean_config.wave_height, 0.0..=2.0)
                .step_by(0.05))
                .changed()
            {
                // Value updated live
            }
            ui.label(format!("  Amplitude: {:.2}m", viewport.ocean_config.wave_height));

            ui.separator();

            // Wave speed control
            ui.label("⚡ Wave Speed:");
            if ui.add(egui::Slider::new(&mut viewport.ocean_config.wave_speed, 0.0..=5.0)
                .step_by(0.1))
                .changed()
            {
                // Value updated live
            }
            ui.label(format!("  Speed: {:.1}x", viewport.ocean_config.wave_speed));

            ui.separator();

            // Animation toggle
            let anim_resp = ui.checkbox(&mut viewport.ocean_config.enable_animation, "▶️ Enable Animation");
            registry.track_widget(&anim_resp, "animation");

            ui.separator();

            // Render layer toggles
            ui.heading("Render Layers");
            ui.separator();

            let water_resp = ui.checkbox(&mut viewport.render_config.show_water, "🌊 Water");
            registry.track_widget(&water_resp, "water");

            let grid_resp = ui.checkbox(&mut viewport.render_config.show_grid, "📐 Grid");
            registry.track_widget(&grid_resp, "grid");

            let axes_resp = ui.checkbox(&mut viewport.render_config.show_axes, "🎯 Axes");
            registry.track_widget(&axes_resp, "axes");

            ui.separator();

            // Pause control
            let pause_resp = ui.checkbox(&mut viewport.paused, "⏸️ Pause Animation");
            registry.track_widget(&pause_resp, "pause");
        });

    registry
}

/// Statistics panel showing ocean metrics
pub fn render_stats_panel(
    egui_ctx: &egui::Context,
    viewport: &mut ViewportState,
    fps: f64,
    cam_pos: cgmath::Point3<f32>,
    active_voxels: usize,
    total_voxels: usize,
    visible_faces: usize,
    system: CoordinateSystem3D,
) -> crate::core::ui_router::UiLayerRegistry {
    let registry = crate::core::ui_router::UiLayerRegistry::default();

    if !viewport.show_stats_panel {
        return registry;
    }

    egui::Window::new("📊 Ocean Statistics")
        .anchor(egui::Align2::RIGHT_TOP, (-10.0, 10.0))
        .default_size([260.0, 380.0])
        .resizable(false)
        .show(egui_ctx, |ui| {
            ui.heading("Statistics");
            ui.separator();

            ui.label(format!("⚡ FPS: {:.0}", fps));
            ui.label(format!(
                "📷 Camera: ({:.1}, {:.1}, {:.1})",
                cam_pos.x, cam_pos.y, cam_pos.z
            ));

            ui.separator();
            ui.heading("Ocean Metrics");
            ui.separator();

            ui.label(format!(
                "🧩 Water Blocks: {}",
                viewport.ocean_config.block_count_x * viewport.ocean_config.block_count_z
            ));
            ui.label(format!(
                "📦 Block: {:.1}m x {:.1}m x {:.1}m",
                viewport.ocean_config.block_world_size,
                viewport.ocean_config.block_world_size,
                viewport.ocean_config.block_world_size
            ));
            ui.label(format!(
                "🔲 Resolution: {}x{}x{} voxels/block",
                viewport.ocean_config.block_size,
                viewport.ocean_config.block_size,
                viewport.ocean_config.block_size
            ));
            ui.label(format!("📐 Voxel Size: {:.2}m", viewport.ocean_config.voxel_size));
            ui.label(format!("💎 Total Voxels: {}", total_voxels));
            ui.label(format!("🌊 Active Water: {}", active_voxels));
            ui.label(format!("🎨 Visible Faces: {}", visible_faces));

            ui.separator();
            ui.heading("Wave Properties");
            ui.separator();

            ui.label(format!(
                "🌊 Height: {:.2}",
                viewport.ocean_config.wave_height
            ));
            ui.label(format!(
                "⚡ Speed: {:.2}x",
                viewport.ocean_config.wave_speed
            ));
            ui.label(format!(
                "▶️ Animated: {}",
                if viewport.ocean_config.enable_animation {
                    "Yes"
                } else {
                    "No"
                }
            ));

            ui.separator();
            ui.label("Cartesian System");
            render_axis_gizmo(ui, system);

            ui.separator();
            ui.horizontal(|ui| {
                ui.colored_label(axis_color(Axis3D::X), "● X");
                ui.colored_label(axis_color(Axis3D::Y), "● Y");
                ui.colored_label(axis_color(Axis3D::Z), "● Z");
            });
        });

    registry
}

/// Controls/Help panel
pub fn render_controls_panel(
    egui_ctx: &egui::Context,
) -> crate::core::ui_router::UiLayerRegistry {
    let registry = crate::core::ui_router::UiLayerRegistry::default();

    egui::Window::new("🎮 Controls")
        .anchor(egui::Align2::RIGHT_BOTTOM, (-10.0, -10.0))
        .default_size([280.0, 220.0])
        .resizable(false)
        .show(egui_ctx, |ui| {
            ui.heading("Navigation Controls");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("");
                ui.label(egui::RichText::new("WASD").monospace());
                ui.label("- Move forward/back/left/right");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("SPACE / SHIFT").monospace());
                ui.label("- Move up/down");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("Q / E").monospace());
                ui.label("- Fast / Slow movement");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("Left Click + Drag").monospace());
                ui.label("- Look around");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("Mouse Wheel").monospace());
                ui.label("- Zoom in/out");
            });
            ui.horizontal(|ui| {
                ui.label("🔹");
                ui.label(egui::RichText::new("ESC").monospace());
                ui.label("- Exit");
            });

            ui.separator();
            ui.small("💡 Tip: Click on UI panels to interact with sliders");
            ui.small("💡 Tip: Click outside panels to capture mouse for camera");
        });

    registry
}

/// Legacy compatibility - simplified viewport overlay
pub fn render_viewport_overlay(
    egui_ctx: &egui::Context,
    viewport: &mut ViewportState,
    system: CoordinateSystem3D,
) -> crate::core::ui_router::UiLayerRegistry {
    let mut registry = crate::core::ui_router::UiLayerRegistry::default();

    egui::Window::new("🎮 Viewport")
        .anchor(egui::Align2::RIGHT_BOTTOM, (-10.0, -10.0))
        .default_size([200.0, 150.0])
        .resizable(false)
        .show(egui_ctx, |ui| {
            ui.heading("Quick Controls");
            ui.separator();

            let config_resp = ui.checkbox(&mut viewport.show_config_panel, "Show Config Panel");
            registry.track_widget(&config_resp, "config_panel");

            let stats_resp = ui.checkbox(&mut viewport.show_stats_panel, "Show Stats Panel");
            registry.track_widget(&stats_resp, "stats_panel");

            ui.separator();

            let water_resp = ui.checkbox(&mut viewport.render_config.show_water, "🌊 Water");
            registry.track_widget(&water_resp, "water");

            let grid_resp = ui.checkbox(&mut viewport.render_config.show_grid, "📐 Grid");
            registry.track_widget(&grid_resp, "grid");

            let axes_resp = ui.checkbox(&mut viewport.render_config.show_axes, "🎯 Axes");
            registry.track_widget(&axes_resp, "axes");

            ui.separator();
            ui.small(format!(
                "Axis: up {} / right {}",
                axis_label(system.up_axis),
                axis_label(system.right_axis()),
            ));
        });

    registry
}
