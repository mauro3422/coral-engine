// Game state - Coral Engine
// Uses OceanWorld as the primary simulation system

use crate::ocean::OceanConfig;
use crate::ocean::OceanWorld;
use crate::ocean::config::DEFAULT_WATER_LAYERS;
use crate::core::scene::{Scene, WorldObjectType};
use crate::render::viewport_api::ViewportState;

pub struct Game {
    pub ocean_world: OceanWorld,
    pub viewport: ViewportState,
    pub scene: Scene,
    pub running: bool,
}

impl Game {
    pub fn new() -> Self {
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
        let mut scene = Scene::standard();

        // Add ocean as a world object
        scene.add_object("Ocean", WorldObjectType::WaterBlock);

        println!(
            "[Game] Ocean initialized: {} blocks, {} active voxels",
            ocean_world.block_count(),
            ocean_world.active_voxel_count()
        );

        Self {
            ocean_world,
            viewport,
            scene,
            running: true,
        }
    }

    pub fn with_config(config: OceanConfig) -> Self {
        let ocean_world = OceanWorld::new(config);
        let viewport = ViewportState::new();
        let scene = Scene::standard();

        Self {
            ocean_world,
            viewport,
            scene,
            running: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if !self.viewport.paused {
            self.ocean_world.update(delta_time);
        }
    }

    pub fn build_scene(&mut self) -> &Scene {
        if let Some(ocean_obj) = self.scene.objects.iter_mut().find(|o| matches!(o.obj_type, WorldObjectType::WaterBlock)) {
            let dims = self.ocean_world.dimensions();
            ocean_obj.scale = [dims.width, dims.water_height, dims.depth];
            ocean_obj.properties.clear();
            ocean_obj.properties.push(
                crate::core::scene::SceneProperty::Float("Wave Height".to_string(), self.ocean_world.config.wave_height)
            );
            ocean_obj.properties.push(
                crate::core::scene::SceneProperty::Float("Wave Speed".to_string(), self.ocean_world.config.wave_speed)
            );
            ocean_obj.properties.push(
                crate::core::scene::SceneProperty::Int("Layers".to_string(), self.ocean_world.config.water_layers as i32)
            );
        }
        &self.scene
    }

    pub fn regenerate_ocean(&mut self) {
        self.ocean_world.generate();
    }
}

impl Default for Game {
    fn default() -> Self { Self::new() }
}
