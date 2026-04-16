// Engine Coordinator - Central singleton orchestrating all subsystems
// Coral Engine v0.4.0

use std::time::Instant;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, KeyCode};
use winit::window::Window;

use crate::common::constants::{
    CAMERA_SPEED_BOOST_MULT, CAMERA_SPEED_SLOW_MULT, DEFAULT_CAMERA_SPEED, MAX_DELTA_TIME,
};
use crate::core::camera::Camera;
use crate::core::database::Database;
use crate::core::input::InputAction;
use crate::core::input::InputManager;
use crate::core::persist::SceneManager;
use crate::core::ui_router::InputRouter;
use crate::game::state::Game;
use crate::render::state::RenderState;
use crate::ui::block_panel::BlockBrowserState;
use crate::ui::editor::EditorState;
use crate::ui::keymap_editor::KeymapEditorState;
use crate::ui::layout::LayoutState;
use crate::ui::layout_editor::LayoutEditorState;
use crate::ui::panel_visibility::PanelVisibility;
use crate::ui::scene_panel::ScenePanelState;

// Use constants from common module (already centralized)

fn key_from_event(key: &winit::event::KeyEvent) -> Option<KeyCode> {
    Some(match key.logical_key.clone() {
        Key::Named(n) => match n {
            winit::keyboard::NamedKey::Space => KeyCode::Space,
            winit::keyboard::NamedKey::Shift => KeyCode::ShiftLeft,
            winit::keyboard::NamedKey::Control => KeyCode::ControlLeft,
            winit::keyboard::NamedKey::Alt => KeyCode::AltLeft,
            winit::keyboard::NamedKey::Escape => KeyCode::Escape,
            winit::keyboard::NamedKey::Tab => KeyCode::Tab,
            winit::keyboard::NamedKey::Enter => KeyCode::Enter,
            winit::keyboard::NamedKey::Backspace => KeyCode::Backspace,
            winit::keyboard::NamedKey::Delete => KeyCode::Delete,
            winit::keyboard::NamedKey::ArrowUp => KeyCode::ArrowUp,
            winit::keyboard::NamedKey::ArrowDown => KeyCode::ArrowDown,
            winit::keyboard::NamedKey::ArrowLeft => KeyCode::ArrowLeft,
            winit::keyboard::NamedKey::ArrowRight => KeyCode::ArrowRight,
            _ => return None,
        },
        Key::Character(c) => match c.to_lowercase().as_str() {
            "w" => KeyCode::KeyW,
            "a" => KeyCode::KeyA,
            "s" => KeyCode::KeyS,
            "d" => KeyCode::KeyD,
            "q" => KeyCode::KeyQ,
            "e" => KeyCode::KeyE,
            "r" => KeyCode::KeyR,
            "f" => KeyCode::KeyF,
            "t" => KeyCode::KeyT,
            "g" => KeyCode::KeyG,
            "y" => KeyCode::KeyY,
            "u" => KeyCode::KeyU,
            "i" => KeyCode::KeyI,
            "o" => KeyCode::KeyO,
            "p" => KeyCode::KeyP,
            "z" => KeyCode::KeyZ,
            "x" => KeyCode::KeyX,
            "c" => KeyCode::KeyC,
            "v" => KeyCode::KeyV,
            "b" => KeyCode::KeyB,
            "n" => KeyCode::KeyN,
            "m" => KeyCode::KeyM,
            "1" => KeyCode::Digit1,
            "2" => KeyCode::Digit2,
            "3" => KeyCode::Digit3,
            "4" => KeyCode::Digit4,
            "5" => KeyCode::Digit5,
            "6" => KeyCode::Digit6,
            "7" => KeyCode::Digit7,
            "8" => KeyCode::Digit8,
            "9" => KeyCode::Digit9,
            "0" => KeyCode::Digit0,
            _ => return None,
        },
        _ => return None,
    })
}

pub struct EngineCoordinator {
    pub camera: Camera,
    pub input: InputManager,
    pub game: Game,
    pub render_state: Option<RenderState>,
    pub input_router: InputRouter,
    pub editor: EditorState,

    pub scene_manager: Option<SceneManager>,
    pub scene_panel: ScenePanelState,
    pub block_browser: BlockBrowserState,
    pub keymap_editor: KeymapEditorState,
    pub layout_editor: LayoutEditorState,
    pub layout: LayoutState,
    pub panel_visibility: PanelVisibility,

    pub egui_winit: Option<egui_winit::State>,
    pub egui_platform: egui::Context,

    block_viz_dirty: bool,
    voxel_mesh_dirty: bool,
    last_frame: Instant,
    fps: f64,
    frame_count: u64,
    fps_timer: f64,
    auto_save_timer: f64,

    pub window: Option<Window>,
}

impl EngineCoordinator {
    pub fn new() -> Self {
        let game = Game::new();

        // Initialize database and scene manager
        let db_path = std::path::Path::new("coral.db");
        let db = Database::new(db_path).ok();
        let scene_manager = db.map(|d| SceneManager::new(d));

        Self {
            camera: Camera::new(),
            input: InputManager::new(),
            game,
            render_state: None,
            input_router: InputRouter::default(),
            editor: EditorState::new(),
            scene_manager,
            scene_panel: ScenePanelState::new(),
            block_browser: BlockBrowserState::new(),
            keymap_editor: KeymapEditorState::new(),
            layout_editor: LayoutEditorState::new(),
            layout: LayoutState::new(),
            panel_visibility: PanelVisibility::new(),
            egui_winit: None,
            egui_platform: egui::Context::default(),
            block_viz_dirty: false,
            voxel_mesh_dirty: true,
            last_frame: Instant::now(),
            fps: 0.0,
            frame_count: 0,
            fps_timer: 0.0,
            auto_save_timer: 0.0,
            window: None,
        }
    }

    pub fn init_egui(&mut self, window: &Window) {
        self.egui_winit = Some(egui_winit::State::new(
            self.egui_platform.clone(),
            egui::ViewportId::ROOT,
            window,
            None,
            None,
            None,
        ));
    }

    pub fn delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now
            .duration_since(self.last_frame)
            .as_secs_f32()
            .min(MAX_DELTA_TIME);
        self.last_frame = now;
        dt
    }

    pub fn update_fps(&mut self, dt: f32) {
        self.fps_timer += dt as f64;
        self.frame_count += 1;
        if self.fps_timer >= 0.5 {
            self.fps = self.frame_count as f64 / self.fps_timer;
            self.frame_count = 0;
            self.fps_timer = 0.0;
        }
    }

    pub fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent) {
        if let (Some(egui_state), Some(window)) = (&mut self.egui_winit, &self.window) {
            let response = egui_state.on_window_event(window, event);
            if response.consumed && !matches!(event, WindowEvent::KeyboardInput { .. }) {
                self.request_redraw();
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(render) = &mut self.render_state {
                    render.resize(*new_size);
                }
                self.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input.cursor_pos = Some((position.x, position.y));
                if self.input.mouse_captured {
                    if let Some(prev) = self.input.prev_cursor_pos {
                        self.input.mouse_delta.0 += position.x - prev.0;
                        self.input.mouse_delta.1 += position.y - prev.1;
                    }
                    self.input.prev_cursor_pos = Some((position.x, position.y));
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll_amount = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                };
                self.camera.zoom(scroll_amount);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let MouseButton::Left = button {
                    let egui_wants_pointer = self.egui_platform.wants_pointer_input();
                    let pointer_over_area = self.egui_platform.is_pointer_over_area();
                    let is_clicking_on_ui = egui_wants_pointer && pointer_over_area;

                    match state {
                        ElementState::Pressed => {
                            if is_clicking_on_ui {
                                // Click on UI widget, don't capture
                                self.input.mouse_captured = false;
                                if let Some(w) = &self.window {
                                    w.set_cursor_visible(true);
                                }
                                return;
                            }
                            // Click on empty space - start capturing immediately
                            self.input.mouse_captured = true;
                            self.input.prev_cursor_pos = self.input.cursor_pos;
                            self.input.mouse_delta = (0.0, 0.0);
                            if let Some(w) = &self.window {
                                w.set_cursor_visible(false);
                            }
                        }
                        ElementState::Released => {
                            self.input.mouse_captured = false;
                            if let Some(w) = &self.window {
                                w.set_cursor_visible(true);
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                let pressed = key_event.state.is_pressed();
                if let Some(key_code) = key_from_event(&key_event) {
                    if pressed {
                        self.input.key_down(key_code);
                    } else {
                        self.input.key_up(key_code);
                    }
                }
                if pressed {
                    match &key_event.logical_key {
                        Key::Named(winit::keyboard::NamedKey::F1) => {
                            self.panel_visibility
                                .toggle(crate::ui::PanelName::KeymapEditor);
                        }
                        Key::Named(winit::keyboard::NamedKey::F2) => {
                            self.panel_visibility
                                .toggle(crate::ui::PanelName::LayoutEditor);
                        }
                        Key::Named(winit::keyboard::NamedKey::Escape) => event_loop.exit(),
                        Key::Character(c) => match c.as_str() {
                            "k" => self
                                .panel_visibility
                                .toggle(crate::ui::PanelName::KeymapEditor),
                            "l" => self
                                .panel_visibility
                                .toggle(crate::ui::PanelName::LayoutEditor),
                            "o" => self.panel_visibility.toggle(crate::ui::PanelName::Outliner),
                            "p" => self
                                .panel_visibility
                                .toggle(crate::ui::PanelName::Properties),
                            "1" => self.panel_visibility.toggle(crate::ui::PanelName::Stats),
                            "2" => self.panel_visibility.toggle(crate::ui::PanelName::Controls),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.tick();
            }
            _ => {}
        }
    }

    fn tick(&mut self) {
        if !self.game.running {
            self.request_redraw();
            return;
        }

        let dt = self.delta_time();
        self.update_fps(dt);

        // Camera
        if self.input.mouse_captured {
            let (dx, dy) = self.input.mouse_delta;
            self.input.mouse_delta = (0.0, 0.0);
            self.camera.rotate(dx, dy);
        }

        let speed_mult = if self.input.is_action_active(InputAction::SpeedBoost) {
            CAMERA_SPEED_BOOST_MULT
        } else if self.input.is_action_active(InputAction::SpeedSlow) {
            CAMERA_SPEED_SLOW_MULT
        } else {
            1.0
        };
        let speed = DEFAULT_CAMERA_SPEED * speed_mult;

        if self.input.is_action_active(InputAction::MoveForward) {
            self.camera.move_forward(dt);
        }
        if self.input.is_action_active(InputAction::MoveBackward) {
            self.camera.move_backward(dt);
        }
        if self.input.is_action_active(InputAction::MoveLeft) {
            self.camera.move_left(dt);
        }
        if self.input.is_action_active(InputAction::MoveRight) {
            self.camera.move_right(dt);
        }
        if self.input.is_action_active(InputAction::MoveUp) {
            self.camera.position.y += speed * dt;
        }
        if self.input.is_action_active(InputAction::MoveDown) {
            self.camera.position.y -= speed * dt;
        }

        self.game.update(dt);
        self.game.build_scene();

        // Auto-save timer (disabled for now until refactored)
        // self.auto_save_timer += dt as f64;

        let animation_enabled = self.game.ocean_world.config.enable_animation;
        let rebuild_needed = self.voxel_mesh_dirty || animation_enabled;

        // Reset voxel mesh dirty after use
        if !animation_enabled {
            self.voxel_mesh_dirty = false;
        }
        self.input_router.reset_frame();

        // Rebuild block viz if dirty
        if self.block_viz_dirty {
            // Refresh scene object scale before rebuilding wireframe
            self.game.build_scene();
            if let Some(rs) = &mut self.render_state {
                rs.rebuild_block_visualization(self.game.ocean_world.dimensions());
            }
            self.block_viz_dirty = false;
        }

        // Extract data before egui borrow
        let active_voxels = self.game.ocean_world.active_voxel_count();
        let _total_voxels = self.game.ocean_world.total_voxels();
        let voxel_size = self.game.ocean_world.config.voxel_size;

        if let (Some(render_state), Some(window)) = (&mut self.render_state, &self.window) {
            let raw_input = self.egui_winit.as_mut().unwrap().take_egui_input(window);

            let full_output = self.egui_platform.run(raw_input, |ctx| {
                let scene = &self.game.scene;

                // Right panel: Outliner + Properties + Scene Manager + Block Browser
                let panel_action = crate::ui::panels::render_right_panel(
                    ctx,
                    scene,
                    &mut self.editor,
                    &mut self.game.ocean_world,
                    &mut self.game.viewport.render_config,
                    self.scene_manager.as_mut(),
                    &mut self.scene_panel,
                    &mut self.game.block_registry,
                    &mut self.block_browser,
                );

                if let Some(id) = panel_action.select_object {
                    self.editor.select_object(id);
                }
                if panel_action.deselect_all {
                    self.editor.deselect_all();
                }
                if panel_action.ocean_rebuild_needed {
                    self.block_viz_dirty = true;
                    self.voxel_mesh_dirty = true;
                    self.scene_panel.mark_dirty();
                }

                // Scene save action
                if let Some(name) = panel_action.scene_save {
                    if let Some(ref mut mgr) = self.scene_manager {
                        if let Err(e) =
                            mgr.save_scene(&name, &self.game.scene, &self.game.block_registry)
                        {
                            eprintln!("[Scene] Save failed: {}", e);
                        } else {
                            println!("[Scene] Saved: {}", name);
                            self.scene_panel.mark_clean();
                        }
                    }
                }

                // Scene load action - log for now, full load needs refactor
                if let Some(name) = panel_action.scene_load {
                    if let Some(ref mut mgr) = self.scene_manager {
                        if mgr.load_scene(&name).ok().flatten().is_some() {
                            println!("[Scene] Ready to load: {} (reload needed)", name);
                        }
                    }
                }

                // Block selection action
                if let Some(pos) = panel_action.select_block {
                    println!("[Block] Selected: {:?}", pos);
                }

                let pv = &self.panel_visibility;

                // Panel visibility: Controls
                if pv.is_visible(crate::ui::PanelName::Controls) {
                    crate::ui::panels::render_controls_panel(ctx);
                }

                // Panel visibility: Stats
                if pv.is_visible(crate::ui::PanelName::Stats) {
                    crate::ui::panels::render_status_bar(
                        ctx,
                        scene,
                        &self.editor,
                        self.fps,
                        active_voxels,
                    );
                }

                // Panel visibility: Keymap Editor
                if pv.is_visible(crate::ui::PanelName::KeymapEditor) {
                    self.keymap_editor.render(ctx, self.input.get_context_map());
                }

                // Panel visibility: Layout Editor
                if pv.is_visible(crate::ui::PanelName::LayoutEditor) {
                    self.layout_editor.render(ctx, &self.layout, pv);
                    for request in self.layout_editor.take_toggle_requests() {
                        self.panel_visibility.toggle(request);
                    }
                }
            });

            // Refresh scene with any ocean changes from egui
            self.game.build_scene();

            let faces = if self.game.viewport.render_config.show_water {
                self.game.ocean_world.visible_faces()
            } else {
                Vec::new()
            };

            self.input_router.set_egui_state(
                self.egui_platform.wants_pointer_input(),
                self.egui_platform.wants_keyboard_input(),
                self.egui_platform.is_pointer_over_area(),
            );
            self.input_router.commit_frame();

            let clipped_meshes = self
                .egui_platform
                .tessellate(full_output.shapes, full_output.pixels_per_point);
            let aspect = window.inner_size().width as f32 / window.inner_size().height as f32;
            let view_proj = self.camera.view_projection(aspect);
            let screen_desc = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [render_state.config.width, render_state.config.height],
                pixels_per_point: full_output.pixels_per_point,
            };

            if let Err(e) = render_state.render(
                &self.game.scene.debug_objects,
                view_proj,
                &clipped_meshes,
                &full_output.textures_delta,
                &screen_desc,
                &faces,
                rebuild_needed,
                self.game.viewport.render_config.show_water,
                false,
                voxel_size,
            ) {
                match e {
                    wgpu::SurfaceError::Lost => eprintln!("[Render] Surface lost"),
                    wgpu::SurfaceError::OutOfMemory => eprintln!("[Render] OOM!"),
                    e => eprintln!("[Render] Error: {:?}", e),
                }
            }
        }

        self.request_redraw();
    }

    pub fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
