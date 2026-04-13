// Engine Coordinator - Central singleton orchestrating all subsystems
// Standardized architecture with clear separation of concerns

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
use crate::render::viewport_api::{render_controls_panel, render_ocean_config_panel, render_stats_panel};

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

    // Egui (owned by coordinator, not render)
    pub egui_winit: Option<egui_winit::State>,
    pub egui_platform: egui::Context,

    // Block visualization dirty flag
    block_viz_dirty: bool,

    // Timing
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

    /// Process window event. Returns true if egui consumed it.
    pub fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent) {
        // Let egui process first (only for mouse events)
        if let (Some(egui_state), Some(window)) = (&mut self.egui_winit, &self.window) {
            let response = egui_state.on_window_event(window, event);
            // Only consume mouse events, NOT keyboard (we need WASD/ESC always)
            if response.consumed && !matches!(event, WindowEvent::KeyboardInput { .. }) {
                self.request_redraw();
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("[Window] Closing...");
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
                        let dx = position.x - prev.0;
                        let dy = position.y - prev.1;
                        self.input.mouse_delta.0 += dx;
                        self.input.mouse_delta.1 += dy;
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
                println!("[Camera] FOV: {:.1}", self.camera.fov);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // Click-and-drag for mouse look
                if let MouseButton::Left = button {
                    let Some(_pointer_pos) = self.input.cursor_pos else {
                        return;
                    };
                    let _pixels_per_point = self
                        .window
                        .as_ref()
                        .map(|w| egui_winit::pixels_per_point(&self.egui_platform, w) as f64)
                        .unwrap_or(1.0);

                    // Use wants_pointer_input as primary check for widget clicks
                    let egui_wants_input = self.egui_platform.wants_pointer_input();

                    match state {
                        ElementState::Pressed => {
                            // If egui wants the pointer, don't capture - let egui handle it
                            if egui_wants_input {
                                self.input_router.release_world_capture();
                                self.input.mouse_captured = false;
                                if let Some(window) = &self.window {
                                    window.set_cursor_visible(true);
                                }
                                // Let the click pass to egui - don't arm world capture
                                return;
                            }

                            // Egui doesn't want it, arm world capture
                            self.input_router.arm_world_capture();
                        }
                        ElementState::Released => {
                            if self.input_router.is_world_active() {
                                self.input_router.release_world_capture();
                                self.input.mouse_captured = false;
                                if let Some(window) = &self.window {
                                    window.set_cursor_visible(true);
                                }
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                let pressed = key_event.state.is_pressed();

                // Handle named keys (Escape, Shift, Space, etc.)
                match &key_event.logical_key {
                    Key::Named(key) => {
                        match key {
                            winit::keyboard::NamedKey::Escape => {
                                if pressed {
                                    println!("[Window] Exiting...");
                                    event_loop.exit();
                                }
                            }
                            winit::keyboard::NamedKey::Space => {
                                if pressed {
                                    self.input.key_down(KeyCode::Space);
                                } else {
                                    self.input.key_up(KeyCode::Space);
                                }
                            }
                            winit::keyboard::NamedKey::Shift => {
                                if pressed {
                                    self.input.key_down(KeyCode::ShiftLeft);
                                } else {
                                    self.input.key_up(KeyCode::ShiftLeft);
                                }
                            }
                            _ => {}
                        }
                    }
                    Key::Character(char) => {
                        match char.as_str() {
                            "w" | "W" => {
                                if pressed {
                                    self.input.key_down(KeyCode::KeyW);
                                } else {
                                    self.input.key_up(KeyCode::KeyW);
                                }
                            }
                            "s" | "S" => {
                                if pressed {
                                    self.input.key_down(KeyCode::KeyS);
                                } else {
                                    self.input.key_up(KeyCode::KeyS);
                                }
                            }
                            "a" | "A" => {
                                if pressed {
                                    self.input.key_down(KeyCode::KeyA);
                                } else {
                                    self.input.key_up(KeyCode::KeyA);
                                }
                            }
                            "d" | "D" => {
                                if pressed {
                                    self.input.key_down(KeyCode::KeyD);
                                } else {
                                    self.input.key_up(KeyCode::KeyD);
                                }
                            }
                            "q" | "Q" => {
                                if pressed {
                                    self.input.key_down(KeyCode::KeyQ);
                                } else {
                                    self.input.key_up(KeyCode::KeyQ);
                                }
                            }
                            "e" | "E" => {
                                if pressed {
                                    self.input.key_down(KeyCode::KeyE);
                                } else {
                                    self.input.key_up(KeyCode::KeyE);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
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

        // Camera rotation
        if self.input.mouse_captured {
            let (dx, dy) = self.input.mouse_delta;
            self.input.mouse_delta = (0.0, 0.0);
            self.camera.rotate(dx, dy);
        }

        // Camera movement
        let speed_mult = if self.input.is_key_pressed(KeyCode::KeyQ) {
            CAMERA_BOOST
        } else if self.input.is_key_pressed(KeyCode::KeyE) {
            CAMERA_SLOW
        } else {
            1.0
        };
        let speed = CAMERA_SPEED * speed_mult;

        if self.input.is_key_pressed(KeyCode::KeyW) {
            self.camera.move_forward(dt);
        }
        if self.input.is_key_pressed(KeyCode::KeyS) {
            self.camera.move_backward(dt);
        }
        if self.input.is_key_pressed(KeyCode::KeyA) {
            self.camera.move_left(dt);
        }
        if self.input.is_key_pressed(KeyCode::KeyD) {
            self.camera.move_right(dt);
        }
        if self.input.is_key_pressed(KeyCode::Space) {
            self.camera.position.y += speed * dt;
        }
        if self.input.is_key_pressed(KeyCode::ShiftLeft) {
            self.camera.position.y -= speed * dt;
        }

        // Update game (ocean animation)
        self.game.update(dt);

        // Build scene
        let scene = self.game.build_scene();

        let rebuild_needed = true; // Always rebuild for water animation

        self.input_router.reset_frame();

        // Rebuild block visualization if dirty (BEFORE render borrow)
        // Uses ocean.dimensions() which derives from ACTUAL config - always in sync
        if self.block_viz_dirty {
            if let Some(rs) = &mut self.render_state {
                let dims = self.game.ocean_world.dimensions();
                rs.rebuild_block_visualization(dims);
            }
            self.block_viz_dirty = false;
        }

        // Render
        if let (Some(render_state), Some(window)) = (&mut self.render_state, &self.window) {
            // Process egui with new viewport API
            let raw_input = self.egui_winit.as_mut().unwrap().take_egui_input(window);

            // Sync viewport config TO ocean world BEFORE egui (so UI shows current values)
            self.game.viewport.ocean_config.voxel_size = self.game.ocean_world.config.voxel_size;
            self.game.viewport.ocean_config.block_world_size = self.game.ocean_world.config.block_world_size;
            self.game.viewport.ocean_config.block_size = self.game.ocean_world.config.block_size;
            self.game.viewport.ocean_config.block_count_x = self.game.ocean_world.config.block_count_x;
            self.game.viewport.ocean_config.block_count_z = self.game.ocean_world.config.block_count_z;
            self.game.viewport.ocean_config.water_layers = self.game.ocean_world.config.water_layers;
            self.game.viewport.ocean_config.wave_height = self.game.ocean_world.config.wave_height;
            self.game.viewport.ocean_config.wave_speed = self.game.ocean_world.config.wave_speed;
            self.game.viewport.ocean_config.enable_animation = self.game.ocean_world.config.enable_animation;
            // Render config is owned by viewport, no sync needed

            // Store config values BEFORE egui to detect changes
            let voxel_size_before = self.game.ocean_world.config.voxel_size;
            let block_world_size_before = self.game.ocean_world.config.block_world_size;
            let water_layers_before = self.game.ocean_world.config.water_layers;
            let wave_height_before = self.game.ocean_world.config.wave_height;
            let wave_speed_before = self.game.ocean_world.config.wave_speed;
            let animation_before = self.game.ocean_world.config.enable_animation;

            let full_output = self.egui_platform.run(raw_input, |ctx| {
                // Config panel - modifies viewport state directly
                let config_registry = render_ocean_config_panel(ctx, &mut self.game.viewport);

                // Stats panel
                let visible_faces_count = if self.game.viewport.render_config.show_water {
                    self.game.ocean_world.active_voxel_count() // Use count, don't collect twice
                } else {
                    0
                };

                let stats_registry = render_stats_panel(
                    ctx,
                    &mut self.game.viewport,
                    self.fps,
                    self.camera.position,
                    self.game.ocean_world.active_voxel_count(),
                    self.game.ocean_world.total_voxels(),
                    visible_faces_count,
                    scene.coordinate_system,
                );

                // Controls panel
                let controls_registry = render_controls_panel(ctx);

                // Merge registries
                for layer in config_registry.layers.iter() {
                    self.input_router.track_layer(layer);
                }
                for layer in stats_registry.layers.iter() {
                    self.input_router.track_layer(layer);
                }
                for layer in controls_registry.layers.iter() {
                    self.input_router.track_layer(layer);
                }
            });

            // Check if ocean config changed
            let voxel_size_changed = (voxel_size_before - self.game.viewport.ocean_config.voxel_size).abs() > f32::EPSILON;
            let block_world_size_changed = (block_world_size_before - self.game.viewport.ocean_config.block_world_size).abs() > f32::EPSILON;
            let water_layers_changed = water_layers_before != self.game.viewport.ocean_config.water_layers;
            let anim_changed = animation_before != self.game.viewport.ocean_config.enable_animation;
            let wave_h_changed = (wave_height_before - self.game.viewport.ocean_config.wave_height).abs() > f32::EPSILON;
            let wave_s_changed = (wave_speed_before - self.game.viewport.ocean_config.wave_speed).abs() > f32::EPSILON;
            let config_changed = voxel_size_changed || block_world_size_changed || water_layers_changed || anim_changed || wave_h_changed || wave_s_changed;
            let geometry_changed = voxel_size_changed || block_world_size_changed || water_layers_changed;

            if config_changed {
                // Apply all changes to ocean world
                self.game.ocean_world.config.voxel_size = self.game.viewport.ocean_config.voxel_size;
                self.game.ocean_world.config.wave_height = self.game.viewport.ocean_config.wave_height;
                self.game.ocean_world.config.wave_speed = self.game.viewport.ocean_config.wave_speed;
                self.game.ocean_world.config.enable_animation = self.game.viewport.ocean_config.enable_animation;

                // Regenerate if geometry changed
                if geometry_changed {
                    self.game.ocean_world.config.block_world_size = self.game.viewport.ocean_config.block_world_size;
                    self.game.ocean_world.config.recalculate_block_size();
                    self.game.ocean_world.config.water_layers = self.game.viewport.ocean_config.water_layers;
                    self.game.ocean_world.generate();
                    self.block_viz_dirty = true; // Mark for rebuild

                    println!(
                        "[Coordinator] Ocean regenerated: voxel_size={}m, block={}m, water_layers={} ({} voxels)",
                        self.game.viewport.ocean_config.voxel_size,
                        self.game.viewport.ocean_config.block_world_size,
                        self.game.viewport.ocean_config.water_layers,
                        self.game.ocean_world.config.block_size
                    );
                } else {
                    println!(
                        "[Coordinator] Config updated: wave_height={:.2}m, wave_speed={:.1}x, anim={}",
                        self.game.viewport.ocean_config.wave_height,
                        self.game.viewport.ocean_config.wave_speed,
                        self.game.viewport.ocean_config.enable_animation
                    );
                }
            }

            // Collect visible faces AFTER egui (in case config changed)
            let faces = if self.game.viewport.render_config.show_water {
                self.game.ocean_world.visible_faces()
            } else {
                Vec::new()
            };

            // Update input router with egui state
            self.input_router.set_egui_state(
                self.egui_platform.wants_pointer_input(),
                self.egui_platform.wants_keyboard_input(),
                self.egui_platform.is_pointer_over_area(),
            );
            self.input_router.commit_frame();

            let pointer_pos = self.input.cursor_pos.unwrap_or((-1.0, -1.0));
            let pixels_per_point = self
                .window
                .as_ref()
                .map(|w| egui_winit::pixels_per_point(&self.egui_platform, w) as f64)
                .unwrap_or(1.0);

            if matches!(
                self.input_router.world,
                crate::core::ui_router::WorldCaptureState::Armed
            ) && self
                .input_router
                .can_arm_world_capture(pointer_pos, pixels_per_point)
            {
                self.input_router.activate_world_capture();
                self.input.mouse_captured = true;
                self.input.prev_cursor_pos = None;
                self.input.mouse_delta = (0.0, 0.0);
                if let Some(window) = &self.window {
                    window.set_cursor_visible(false);
                }
            } else if self
                .input_router
                .blocks_world_pointer(pointer_pos, pixels_per_point)
            {
                self.input_router.release_world_capture();
                self.input.mouse_captured = false;
                if let Some(window) = &self.window {
                    window.set_cursor_visible(true);
                }
            }

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
                &scene,
                view_proj,
                &clipped_meshes,
                &full_output.textures_delta,
                &screen_desc,
                &faces,
                rebuild_needed,
                self.game.viewport.render_config.show_water,
                false, // No terrain in new architecture
                self.game.ocean_world.config.voxel_size,
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
