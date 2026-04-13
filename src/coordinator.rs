// Engine Coordinator - Central singleton orchestrating all subsystems
// Coral Engine v0.4.0

use std::time::Instant;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, KeyCode};
use winit::window::Window;

use crate::core::camera::Camera;
use crate::core::input::InputState;
use crate::core::ui_router::InputRouter;
use crate::game::state::Game;
use crate::render::state::RenderState;
use crate::ui::editor::EditorState;

// Constants
const CAMERA_SPEED: f32 = 15.0;
const CAMERA_BOOST: f32 = 2.0;
const CAMERA_SLOW: f32 = 0.33;
const MAX_DELTA_TIME: f32 = 0.05;

pub struct EngineCoordinator {
    pub camera: Camera,
    pub input: InputState,
    pub game: Game,
    pub render_state: Option<RenderState>,
    pub input_router: InputRouter,
    pub editor: EditorState,

    pub egui_winit: Option<egui_winit::State>,
    pub egui_platform: egui::Context,

    block_viz_dirty: bool,
    last_frame: Instant,
    fps: f64,
    frame_count: u64,
    fps_timer: f64,

    pub window: Option<Window>,
}

impl EngineCoordinator {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            input: InputState::new(),
            game: Game::new(),
            render_state: None,
            input_router: InputRouter::default(),
            editor: EditorState::new(),
            egui_winit: None,
            egui_platform: egui::Context::default(),
            block_viz_dirty: false,
            last_frame: Instant::now(),
            fps: 0.0,
            frame_count: 0,
            fps_timer: 0.0,
            window: None,
        }
    }

    pub fn init_egui(&mut self, window: &Window) {
        self.egui_winit = Some(egui_winit::State::new(
            self.egui_platform.clone(),
            egui::ViewportId::ROOT,
            window,
            None, None, None,
        ));
    }

    pub fn delta_time(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f32().min(MAX_DELTA_TIME);
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
            WindowEvent::CloseRequested => { event_loop.exit(); }
            WindowEvent::Resized(new_size) => {
                if let Some(render) = &mut self.render_state { render.resize(*new_size); }
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
                                if let Some(w) = &self.window { w.set_cursor_visible(true); }
                                return;
                            }
                            // Click on empty space - start capturing immediately
                            self.input.mouse_captured = true;
                            self.input.prev_cursor_pos = self.input.cursor_pos;
                            self.input.mouse_delta = (0.0, 0.0);
                            if let Some(w) = &self.window { w.set_cursor_visible(false); }
                        }
                        ElementState::Released => {
                            self.input.mouse_captured = false;
                            if let Some(w) = &self.window { w.set_cursor_visible(true); }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                let pressed = key_event.state.is_pressed();
                match &key_event.logical_key {
                    Key::Named(key) => match key {
                        winit::keyboard::NamedKey::Escape if pressed => event_loop.exit(),
                        winit::keyboard::NamedKey::Space => {
                            if pressed { self.input.key_down(KeyCode::Space); }
                            else { self.input.key_up(KeyCode::Space); }
                        }
                        winit::keyboard::NamedKey::Shift => {
                            if pressed { self.input.key_down(KeyCode::ShiftLeft); }
                            else { self.input.key_up(KeyCode::ShiftLeft); }
                        }
                        _ => {}
                    }
                    Key::Character(char) => {
                        match char.as_str() {
                            "w" | "W" => if pressed { self.input.key_down(KeyCode::KeyW); } else { self.input.key_up(KeyCode::KeyW); }
                            "a" | "A" => if pressed { self.input.key_down(KeyCode::KeyA); } else { self.input.key_up(KeyCode::KeyA); }
                            "s" | "S" => if pressed { self.input.key_down(KeyCode::KeyS); } else { self.input.key_up(KeyCode::KeyS); }
                            "d" | "D" => if pressed { self.input.key_down(KeyCode::KeyD); } else { self.input.key_up(KeyCode::KeyD); }
                            "q" | "Q" => if pressed { self.input.key_down(KeyCode::KeyQ); } else { self.input.key_up(KeyCode::KeyQ); }
                            "e" | "E" => if pressed { self.input.key_down(KeyCode::KeyE); } else { self.input.key_up(KeyCode::KeyE); }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => { self.tick(); }
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

        let speed_mult = if self.input.is_key_pressed(KeyCode::KeyQ) { CAMERA_BOOST }
            else if self.input.is_key_pressed(KeyCode::KeyE) { CAMERA_SLOW }
            else { 1.0 };
        let speed = CAMERA_SPEED * speed_mult;

        if self.input.is_key_pressed(KeyCode::KeyW) { self.camera.move_forward(dt); }
        if self.input.is_key_pressed(KeyCode::KeyS) { self.camera.move_backward(dt); }
        if self.input.is_key_pressed(KeyCode::KeyA) { self.camera.move_left(dt); }
        if self.input.is_key_pressed(KeyCode::KeyD) { self.camera.move_right(dt); }
        if self.input.is_key_pressed(KeyCode::Space) { self.camera.position.y += speed * dt; }
        if self.input.is_key_pressed(KeyCode::ShiftLeft) { self.camera.position.y -= speed * dt; }

        self.game.update(dt);
        self.game.build_scene(); // Update scene objects

        let rebuild_needed = true;
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
        let total_voxels = self.game.ocean_world.total_voxels();
        let voxel_size = self.game.ocean_world.config.voxel_size;

        if let (Some(render_state), Some(window)) = (&mut self.render_state, &self.window) {
            let raw_input = self.egui_winit.as_mut().unwrap().take_egui_input(window);

            let full_output = self.egui_platform.run(raw_input, |ctx| {
                let scene = &self.game.scene;

                // Right panel: Outliner + Properties (stacked, side by side with viewport)
                let panel_action = crate::ui::panels::render_right_panel(
                    ctx, scene, &mut self.editor,
                    &mut self.game.ocean_world,
                    &mut self.game.viewport.render_config,
                );

                if let Some(id) = panel_action.select_object { self.editor.select_object(id); }
                if panel_action.deselect_all { self.editor.deselect_all(); }
                if panel_action.ocean_rebuild_needed { self.block_viz_dirty = true; }

                // Controls panel
                crate::ui::panels::render_controls_panel(ctx);

                // Status bar
                crate::ui::panels::render_status_bar(ctx, scene, &self.editor, self.fps, active_voxels);
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

            let clipped_meshes = self.egui_platform.tessellate(full_output.shapes, full_output.pixels_per_point);
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
        if let Some(window) = &self.window { window.request_redraw(); }
    }
}
