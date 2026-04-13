// Game state - Simplified ocean-only architecture
// Uses OceanWorld as the primary simulation system

use crate::ocean::OceanConfig;
use crate::ocean::OceanWorld;
use crate::ocean::config::DEFAULT_WATER_LAYERS;
use crate::core::scene::{Scene, SceneMesh};
use crate::render::viewport_api::ViewportState;

pub struct Game {
    pub ocean_world: OceanWorld,
    pub viewport: ViewportState,
    pub running: bool,
}

impl Game {
    pub fn new() -> Self {
        // Create default ocean config: single 4m block with 0.5m voxels (8x8x8)
        let ocean_config = OceanConfig::builder()
            .voxel_size(0.5)
            .block_world_size(4.0)
            .blocks_x(1)
            .blocks_z(1)
            .water_layers(DEFAULT_WATER_LAYERS)
            .wave_height(0.3)
            .wave_speed(1.0)
            .enable_animation(true)
            .build();

        let ocean_world = OceanWorld::new(ocean_config);
        let viewport = ViewportState::new();

        println!(
            "[Game] Ocean initialized: {} chunks, {} active voxels",
            ocean_world.block_count(),
            ocean_world.active_voxel_count()
        );

        Self {
            ocean_world,
            viewport,
            running: true,
        }
    }

    /// Create game with custom ocean configuration
    pub fn with_config(config: OceanConfig) -> Self {
        let ocean_world = OceanWorld::new(config);
        let viewport = ViewportState::new();

        Self {
            ocean_world,
            viewport,
            running: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.viewport.paused {
            self.ocean_world.update(delta_time);
        }
    }

    pub fn build_scene(&self) -> Scene {
        let mut scene = Scene::standard();

        // Add debug visualization
        if self.viewport.render_config.show_grid {
            scene.push_identity(SceneMesh::Grid);
        }
        if self.viewport.render_config.show_axes {
            scene.push_identity(SceneMesh::Axes);
        }

        scene
    }

    /// Regenerate ocean with current config
    pub fn regenerate_ocean(&mut self) {
        self.ocean_world.generate();
        println!(
            "[Game] Ocean regenerated: {} chunks",
            self.ocean_world.block_count()
        );
    }

    /// Update ocean configuration and regenerate
    pub fn update_ocean_config(&mut self, config: OceanConfig) {
        self.ocean_world.config = config;
        self.regenerate_ocean();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
