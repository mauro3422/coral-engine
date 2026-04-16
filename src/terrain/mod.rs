//! Terrain System - Coral Engine
//! Voxel terrain generation and management

mod render;

use crate::common::constants::{
    DEFAULT_TERRAIN_HEIGHT, DEFAULT_TERRAIN_NOISE_SCALE, DEFAULT_TERRAIN_SEA_LEVEL,
    DEFAULT_TERRAIN_SEED, DEFAULT_TERRAIN_SIZE_X, DEFAULT_TERRAIN_SIZE_Y, DEFAULT_TERRAIN_SIZE_Z,
    DEFAULT_TERRAIN_VOXEL_SIZE,
};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
pub use self::render::TerrainRender;

pub type TerrainPos = (i32, i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainBlock {
    Air,
    Dirt,
    Grass,
    Stone,
    Sand,
    Gravel,
    Clay,
}

impl TerrainBlock {
    pub fn is_solid(&self) -> bool {
        !matches!(self, TerrainBlock::Air)
    }

    pub fn color(&self) -> [f32; 3] {
        match self {
            TerrainBlock::Air => [0.0, 0.0, 0.0],
            TerrainBlock::Dirt => [0.4, 0.25, 0.1],
            TerrainBlock::Grass => [0.2, 0.5, 0.15],
            TerrainBlock::Stone => [0.5, 0.5, 0.5],
            TerrainBlock::Sand => [0.85, 0.75, 0.55],
            TerrainBlock::Gravel => [0.4, 0.38, 0.35],
            TerrainBlock::Clay => [0.6, 0.35, 0.25],
        }
    }

    pub fn from_height(height: f32, surface_height: f32) -> Self {
        if height > surface_height + 3.0 {
            TerrainBlock::Stone
        } else if height > surface_height + 1.0 {
            TerrainBlock::Dirt
        } else if height > surface_height - 0.5 {
            TerrainBlock::Grass
        } else {
            TerrainBlock::Air
        }
    }
}

impl Default for TerrainBlock {
    fn default() -> Self {
        TerrainBlock::Air
    }
}

#[derive(Clone, Debug, Default)]
pub struct TerrainVoxel {
    pub block: TerrainBlock,
    pub light: f32,
}

impl TerrainVoxel {
    pub fn new(block: TerrainBlock) -> Self {
        Self { block, light: 1.0 }
    }
}

#[derive(Clone, Debug)]
pub struct TerrainConfig {
    pub size_x: u32,
    pub size_y: u32,
    pub size_z: u32,
    pub voxel_size: f32,
    pub sea_level: f32,
    pub terrain_height: f32,
    pub noise_scale: f32,
    pub seed: u32,
}

impl TerrainConfig {
    pub fn new() -> Self {
        Self::default()
    }

    // Getters
    pub fn size_x(&self) -> u32 {
        self.size_x
    }
    pub fn size_y(&self) -> u32 {
        self.size_y
    }
    pub fn size_z(&self) -> u32 {
        self.size_z
    }
    pub fn voxel_size(&self) -> f32 {
        self.voxel_size
    }
    pub fn sea_level(&self) -> f32 {
        self.sea_level
    }
    pub fn terrain_height(&self) -> f32 {
        self.terrain_height
    }
    pub fn noise_scale(&self) -> f32 {
        self.noise_scale
    }
    pub fn seed(&self) -> u32 {
        self.seed
    }

    // Setters
    pub fn set_size(&mut self, x: u32, y: u32, z: u32) {
        self.size_x = x.clamp(1, 256);
        self.size_y = y.clamp(1, 128);
        self.size_z = z.clamp(1, 256);
    }

    pub fn set_terrain_height(&mut self, height: f32) {
        self.terrain_height = height.clamp(1.0, 64.0);
    }

    pub fn set_sea_level(&mut self, level: f32) {
        self.sea_level = level.clamp(0.0, 64.0);
    }

    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
    }

    pub fn set_noise_scale(&mut self, scale: f32) {
        self.noise_scale = scale.clamp(0.01, 0.2);
    }

    pub fn builder() -> TerrainConfigBuilder {
        TerrainConfigBuilder::new()
    }
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            size_x: DEFAULT_TERRAIN_SIZE_X,
            size_y: DEFAULT_TERRAIN_SIZE_Y,
            size_z: DEFAULT_TERRAIN_SIZE_Z,
            voxel_size: DEFAULT_TERRAIN_VOXEL_SIZE,
            sea_level: DEFAULT_TERRAIN_SEA_LEVEL,
            terrain_height: DEFAULT_TERRAIN_HEIGHT,
            noise_scale: DEFAULT_TERRAIN_NOISE_SCALE,
            seed: DEFAULT_TERRAIN_SEED,
        }
    }
}

pub struct TerrainConfigBuilder {
    config: TerrainConfig,
}

impl TerrainConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: TerrainConfig::default(),
        }
    }

    pub fn size(mut self, x: u32, y: u32, z: u32) -> Self {
        self.config.size_x = x.clamp(1, 256);
        self.config.size_y = y.clamp(1, 128);
        self.config.size_z = z.clamp(1, 256);
        self
    }

    pub fn terrain_height(mut self, height: f32) -> Self {
        self.config.terrain_height = height.clamp(1.0, 64.0);
        self
    }

    pub fn sea_level(mut self, level: f32) -> Self {
        self.config.sea_level = level.clamp(0.0, 64.0);
        self
    }

    pub fn seed(mut self, seed: u32) -> Self {
        self.config.seed = seed;
        self
    }

    pub fn noise_scale(mut self, scale: f32) -> Self {
        self.config.noise_scale = scale.clamp(0.01, 0.2);
        self
    }

    pub fn build(self) -> TerrainConfig {
        self.config
    }
}

#[derive(Clone, Debug)]
pub struct TerrainWorld {
    pub config: TerrainConfig,
    voxels: Vec<TerrainVoxel>,
}

impl TerrainWorld {
    pub fn new(config: TerrainConfig) -> Self {
        let total = (config.size_x * config.size_y * config.size_z) as usize;
        Self {
            config,
            voxels: vec![TerrainVoxel::default(); total],
        }
    }

    pub fn generate(&mut self) {
        self.generate_terrain();
    }

    fn generate_terrain(&mut self) {
        let (sx, sy, sz, terrain_height) = {
            let config = &self.config;
            (
                config.size_x as i32,
                config.size_y as i32,
                config.size_z as i32,
                config.terrain_height,
            )
        };

        for z in 0..sz {
            for x in 0..sx {
                let noise_val = self.get_noise(x, z);
                let surface = (terrain_height * noise_val) as i32;

                for y in 0..sy {
                    let height_diff = y as i32 - surface;

                    let block = if height_diff < -2 {
                        TerrainBlock::Air
                    } else if height_diff < 0 {
                        TerrainBlock::Sand
                    } else if height_diff < 1 {
                        TerrainBlock::Grass
                    } else if height_diff < 4 {
                        TerrainBlock::Dirt
                    } else {
                        TerrainBlock::Stone
                    };

                    self.set_voxel(x as u32, y as u32, z as u32, TerrainVoxel::new(block));
                }
            }
        }
    }

    fn get_noise(&self, x: i32, z: i32) -> f32 {
        let seed = self.config.seed as f32;
        let scale = self.config.noise_scale;

        let nx = (x as f32 * scale).sin() * 100.0 + seed;
        let nz = (z as f32 * scale).cos() * 100.0 + seed;

        let n1 = (nx * 0.5 + nz * 0.3).sin().abs();
        let n2 = (nx * 0.3 - nz * 0.5).sin().abs();
        let n3 = ((nx + nz) * 0.2).sin().abs();

        (n1 + n2 + n3) / 3.0
    }

    pub fn get_voxel(&self, x: u32, y: u32, z: u32) -> Option<&TerrainVoxel> {
        let config = &self.config;
        if x >= config.size_x || y >= config.size_y || z >= config.size_z {
            return None;
        }
        let idx = (z * config.size_x * config.size_y + y * config.size_x + x) as usize;
        self.voxels.get(idx)
    }

    pub fn set_voxel(&mut self, x: u32, y: u32, z: u32, voxel: TerrainVoxel) {
        let config = &self.config;
        if x >= config.size_x || y >= config.size_y || z >= config.size_z {
            return;
        }
        let idx = (z * config.size_x * config.size_y + y * config.size_x + x) as usize;
        if idx < self.voxels.len() {
            self.voxels[idx] = voxel;
        }
    }

    pub fn is_solid(&self, x: u32, y: u32, z: u32) -> bool {
        self.get_voxel(x, y, z)
            .map(|v| v.block.is_solid())
            .unwrap_or(false)
    }

    pub fn dimensions(&self) -> (u32, u32, u32) {
        (self.config.size_x, self.config.size_y, self.config.size_z)
    }

    pub fn total_solid_voxels(&self) -> usize {
        self.voxels.iter().filter(|v| v.block.is_solid()).count()
    }

    pub fn total_voxels(&self) -> usize {
        self.voxels.len()
    }
}

impl crate::common::VoxelWorld for TerrainWorld {
    fn total_voxels(&self) -> usize {
        self.total_voxels()
    }

    fn active_voxels(&self) -> usize {
        self.total_solid_voxels()
    }

    fn is_solid(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || y < 0 || z < 0 {
            return false;
        }
        self.is_solid(x as u32, y as u32, z as u32)
    }
}
