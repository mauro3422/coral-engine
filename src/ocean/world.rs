// OceanWorld - Manages water blocks and animation

use std::collections::HashMap;
use crate::ocean::config::OceanConfig;
use crate::ocean::block::{BlockPos, WaterBlock};
use crate::ocean::render::WaterFace;
use crate::ocean::bounds::ObjectBounds;

/// Standard dimensions for the ocean - always derived from actual config
/// This is the canonical way to get ocean dimensions for wireframes, collision, etc.
#[derive(Clone, Copy, Debug)]
pub struct OceanDimensions {
    pub width: f32,
    pub water_height: f32,
    pub depth: f32,
    pub voxel_size: f32,
    pub voxels_per_axis: u32,
    pub water_layers: u32,
}

impl OceanDimensions {
    pub fn from_config(config: &OceanConfig) -> Self {
        Self {
            width: config.block_count_x as f32 * config.block_size as f32 * config.voxel_size,
            depth: config.block_count_z as f32 * config.block_size as f32 * config.voxel_size,
            water_height: config.water_layers as f32 * config.voxel_size,
            voxel_size: config.voxel_size,
            voxels_per_axis: config.block_size,
            water_layers: config.water_layers,
        }
    }

    /// Convert to standard ObjectBounds for wireframe/collision
    /// Water sits ON TOP of grid (Y=0), extends upward
    pub fn bounds(&self) -> ObjectBounds {
        let half = self.width / 2.0;
        ObjectBounds::from_origin_and_size(
            [-half, 0.0, -half],
            [self.width, self.water_height, self.depth],
        )
    }
}

pub struct OceanWorld {
    pub config: OceanConfig,
    pub blocks: HashMap<BlockPos, WaterBlock>,
    pub time: f32,
}

impl OceanWorld {
    pub fn new(config: OceanConfig) -> Self {
        let mut ocean = Self {
            config: config.clone(),
            blocks: HashMap::new(),
            time: 0.0,
        };
        ocean.generate();
        ocean
    }

    pub fn generate(&mut self) {
        let mut total_water_voxels = 0usize;

        for bx in 0..self.config.block_count_x {
            for bz in 0..self.config.block_count_z {
                let pos = BlockPos::new(bx, bz);
                let mut block = WaterBlock::new(
                    pos,
                    self.config.block_size,
                    self.config.voxel_size,
                );

                block.fill_with_surface_water(self.config.water_layers);
                block.wave_amplitude = self.config.wave_height;
                block.dirty = true;

                total_water_voxels += block.voxels.iter().filter(|v| v.is_water()).count();
                self.blocks.insert(pos, block);
            }
        }

        let (wx, wz) = (
            self.config.block_count_x as f32 * self.config.block_size as f32 * self.config.voxel_size,
            self.config.block_count_z as f32 * self.config.block_size as f32 * self.config.voxel_size,
        );
        println!(
            "[OceanWorld] Generated {} water blocks ({}x{} voxels, voxel={}m, layers={})",
            self.blocks.len(),
            self.config.block_size,
            self.config.block_size,
            self.config.voxel_size,
            self.config.water_layers,
        );
        println!(
            "  World: {:.1}m x {:.1}m | Water voxels: {}",
            wx, wz, total_water_voxels
        );
    }

    pub fn update(&mut self, dt: f32) {
        if !self.config.enable_animation { return }
        self.time += dt * self.config.wave_speed;
        for block in self.blocks.values_mut() {
            block.wave_amplitude = self.config.wave_height;
            block.dirty = true;
        }
    }

    pub fn visible_faces(&self) -> Vec<WaterFace> {
        let mut all_faces = Vec::new();
        for block in self.blocks.values() {
            let faces = block.collect_visible_faces(self.time, self.config.enable_animation);
            all_faces.extend(faces);
        }
        all_faces
    }

    pub fn block_count(&self) -> usize { self.blocks.len() }

    pub fn total_voxels(&self) -> usize {
        self.blocks.len() * (self.config.block_size.pow(3) as usize)
    }

    pub fn active_voxel_count(&self) -> usize {
        self.blocks.values()
            .map(|b| b.voxels.iter().filter(|v| v.is_water()).count())
            .sum()
    }

    /// Get current dimensions derived from config - ALWAYS in sync
    pub fn dimensions(&self) -> OceanDimensions {
        OceanDimensions::from_config(&self.config)
    }
}
