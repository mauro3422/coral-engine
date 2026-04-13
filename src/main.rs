mod coordinator;
mod core;
mod game;
mod ocean;
mod render;

use coordinator::EngineCoordinator;
use render::state::RenderState;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowAttributes,
};

struct App {
    coordinator: EngineCoordinator,
}

impl App {
    fn new() -> Self {
        Self {
            coordinator: EngineCoordinator::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.coordinator.window.is_some() {
            return;
        }

        let window_attributes = WindowAttributes::default()
            .with_title("Motor Grafico v0.4.0 - Ocean Engine")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .with_visible(true);

        let window = match event_loop.create_window(window_attributes) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("[Window] Failed: {}", e);
                event_loop.exit();
                return;
            }
        };

        let size = window.inner_size();
        println!("[Window] Created: {}x{}", size.width, size.height);

        let mut render_state = pollster::block_on(async { RenderState::new(&window).await });

        // Build initial block visualization using ocean dimensions (always in sync)
        render_state.rebuild_block_visualization(
            self.coordinator.game.ocean_world.dimensions()
        );

        self.coordinator.init_egui(&window);
        self.coordinator.window = Some(window);
        self.coordinator.render_state = Some(render_state);

        println!("[App] Ready! Click = mouse look, ESC = exit");
        self.coordinator.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.coordinator.handle_event(event_loop, &event);
    }
}

fn main() {
    println!("=== Motor Grafico v0.4.0 - Ocean Engine ===");
    println!("[Core] Initializing ocean simulation...");
    println!("[Core] Starting...\n");
    println!("[Controls]");
    println!("  WASD - Move | Space/Shift - Up/Down");
    println!("  Q - Fast | E - Slow");
    println!("  Hold Left Click - Look around");
    println!("  ESC - Exit\n");
    println!("[Features]");
    println!("  🌊 8x8x8 Voxel Ocean with wave animation");
    println!("  📊 Real-time statistics panel");
    println!("  ⚙️ Configurable chunk size and wave properties\n");

    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);

    println!("[Core] Shutdown complete!");
}
