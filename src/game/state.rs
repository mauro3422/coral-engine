// Game state - Coral Engine
// Uses OceanWorld + BlockRegistry as the primary simulation system

use crate::core::scene::{Scene, WorldObjectType};
use crate::ocean::config::DEFAULT_WATER_LAYERS;
use crate::ocean::{BlockMetadata, BlockRegistry, BlockTag, OceanConfig, OceanWorld};
use crate::render::viewport_api::ViewportState;

pub struct Game {
    pub ocean_world: OceanWorld,
    pub block_registry: BlockRegistry,
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

        let ocean_world = OceanWorld::new(ocean_config.clone());
        let viewport = ViewportState::new();
        let mut scene = Scene::standard();
        let mut block_registry = BlockRegistry::new();

        // Add each water block as separate WorldObject with registry
        for bx in 0..ocean_world.config.block_count_x {
            for bz in 0..ocean_world.config.block_count_z {
                let name = format!("WaterBlock[{}, {}]", bx, bz);
                let block_id = scene.add_object(&name, WorldObjectType::WaterBlock);
                if let Some(obj) = scene.get_object_mut(block_id) {
                    obj.position = [
                        (bx * ocean_world.config.block_size as i32) as f32
                            * ocean_world.config.voxel_size,
                        0.0,
                        (bz * ocean_world.config.block_size as i32) as f32
                            * ocean_world.config.voxel_size,
                    ];
                }

                // Register in BlockRegistry
                let pos = crate::ocean::BlockPos::new(bx, bz);
                if ocean_world.blocks.get(&pos).is_some() {
                    let meta = BlockMetadata::new(name, pos)
                        .with_tag(BlockTag::new("water"))
                        .with_tag(BlockTag::new("ocean"));
                    block_registry.metadata.insert(pos, meta);
                }
            }
        }

        println!(
            "[Game] Ocean initialized: {} blocks, {} active voxels",
            ocean_world.block_count(),
            ocean_world.active_voxel_count()
        );

        Self {
            ocean_world,
            block_registry,
            viewport,
            scene,
            running: true,
        }
    }

    pub fn with_config(config: OceanConfig) -> Self {
        let ocean_world = OceanWorld::new(config);
        let viewport = ViewportState::new();
        let scene = Scene::standard();
        let block_registry = BlockRegistry::new();

        Self {
            ocean_world,
            block_registry,
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
        if let Some(ocean_obj) = self
            .scene
            .objects
            .iter_mut()
            .find(|o| matches!(o.obj_type, WorldObjectType::WaterBlock))
        {
            let dims = self.ocean_world.dimensions();
            ocean_obj.scale = [dims.width, dims.water_height, dims.depth];
            ocean_obj.properties.clear();
            ocean_obj
                .properties
                .push(crate::core::scene::SceneProperty::Float(
                    "Wave Height".to_string(),
                    self.ocean_world.config.wave_height,
                ));
            ocean_obj
                .properties
                .push(crate::core::scene::SceneProperty::Float(
                    "Wave Speed".to_string(),
                    self.ocean_world.config.wave_speed,
                ));
            ocean_obj
                .properties
                .push(crate::core::scene::SceneProperty::Int(
                    "Layers".to_string(),
                    self.ocean_world.config.water_layers as i32,
                ));
        }
        &self.scene
    }

    pub fn regenerate_ocean(&mut self) {
        self.ocean_world.generate();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
