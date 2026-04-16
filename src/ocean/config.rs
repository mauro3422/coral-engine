// Ocean configuration - canonical definition
// Provides builder pattern with validation for ocean simulation parameters

pub use crate::common::constants::{
    DEFAULT_BLOCK_COUNT_X, DEFAULT_BLOCK_COUNT_Z, DEFAULT_BLOCK_WORLD_SIZE, DEFAULT_VOXEL_SIZE,
    DEFAULT_WATER_COLOR, DEFAULT_WATER_LAYERS, DEFAULT_WAVE_HEIGHT, DEFAULT_WAVE_SPEED,
    MAX_BLOCK_COUNT, MAX_BLOCK_SIZE, MAX_BLOCK_WORLD_SIZE, MAX_VOXEL_SIZE, MAX_WATER_LAYERS,
    MAX_WAVE_HEIGHT, MAX_WAVE_SPEED, MIN_BLOCK_SIZE, MIN_BLOCK_WORLD_SIZE, MIN_VOXEL_SIZE,
    MIN_WATER_LAYERS,
};

#[derive(Clone, Debug, Default)]
pub struct OceanConfig {
    pub voxel_size: f32,
    pub block_world_size: f32,
    pub block_size: u32,
    pub block_count_x: i32,
    pub block_count_z: i32,
    pub water_layers: u32,
    pub wave_height: f32,
    pub wave_speed: f32,
    pub water_color: [f32; 3],
    pub enable_animation: bool,
}

impl OceanConfig {
    pub fn default() -> Self {
        let mut cfg = Self {
            voxel_size: DEFAULT_VOXEL_SIZE,
            block_world_size: DEFAULT_BLOCK_WORLD_SIZE,
            block_size: 0,
            block_count_x: DEFAULT_BLOCK_COUNT_X,
            block_count_z: DEFAULT_BLOCK_COUNT_Z,
            water_layers: DEFAULT_WATER_LAYERS,
            wave_height: DEFAULT_WAVE_HEIGHT,
            wave_speed: DEFAULT_WAVE_SPEED,
            water_color: DEFAULT_WATER_COLOR,
            enable_animation: true,
        };
        cfg.recalculate_block_size();
        cfg
    }

    // Getters
    pub fn voxel_size(&self) -> f32 {
        self.voxel_size
    }
    pub fn block_world_size(&self) -> f32 {
        self.block_world_size
    }
    pub fn block_size(&self) -> u32 {
        self.block_size
    }
    pub fn block_count_x(&self) -> i32 {
        self.block_count_x
    }
    pub fn block_count_z(&self) -> i32 {
        self.block_count_z
    }
    pub fn water_layers(&self) -> u32 {
        self.water_layers
    }
    pub fn wave_height(&self) -> f32 {
        self.wave_height
    }
    pub fn wave_speed(&self) -> f32 {
        self.wave_speed
    }
    pub fn water_color(&self) -> [f32; 3] {
        self.water_color
    }
    pub fn enable_animation(&self) -> bool {
        self.enable_animation
    }

    // Setters with validation
    pub fn set_voxel_size(&mut self, size: f32) {
        self.voxel_size = size.clamp(MIN_VOXEL_SIZE, MAX_VOXEL_SIZE);
        self.recalculate_block_size();
    }

    pub fn set_block_world_size(&mut self, size: f32) {
        self.block_world_size = size.clamp(MIN_BLOCK_WORLD_SIZE, MAX_BLOCK_WORLD_SIZE);
        self.recalculate_block_size();
    }

    pub fn set_block_count_x(&mut self, count: i32) {
        self.block_count_x = count.clamp(1, MAX_BLOCK_COUNT);
    }

    pub fn set_block_count_z(&mut self, count: i32) {
        self.block_count_z = count.clamp(1, MAX_BLOCK_COUNT);
    }

    pub fn set_water_layers(&mut self, layers: u32) {
        self.water_layers = layers.clamp(MIN_WATER_LAYERS, MAX_WATER_LAYERS);
    }

    pub fn set_wave_height(&mut self, height: f32) {
        self.wave_height = height.clamp(0.0, MAX_WAVE_HEIGHT);
    }

    pub fn set_wave_speed(&mut self, speed: f32) {
        self.wave_speed = speed.clamp(0.0, MAX_WAVE_SPEED);
    }

    pub fn set_enable_animation(&mut self, enabled: bool) {
        self.enable_animation = enabled;
    }

    pub fn recalculate_block_size(&mut self) {
        self.block_size = (self.block_world_size / self.voxel_size).round() as u32;
        self.block_size = self.block_size.clamp(MIN_BLOCK_SIZE, MAX_BLOCK_SIZE);
    }

    pub fn builder() -> OceanConfigBuilder {
        OceanConfigBuilder::new()
    }
}

pub struct OceanConfigBuilder {
    config: OceanConfig,
}

impl OceanConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: OceanConfig::default(),
        }
    }

    pub fn voxel_size(mut self, size: f32) -> Self {
        self.config.voxel_size = size.clamp(MIN_VOXEL_SIZE, MAX_VOXEL_SIZE);
        self.config.recalculate_block_size();
        self
    }

    pub fn block_world_size(mut self, size: f32) -> Self {
        self.config.block_world_size = size.clamp(1.0, 16.0);
        self.config.recalculate_block_size();
        self
    }

    pub fn blocks_x(mut self, count: i32) -> Self {
        self.config.block_count_x = count.clamp(1, 16);
        self
    }

    pub fn blocks_z(mut self, count: i32) -> Self {
        self.config.block_count_z = count.clamp(1, 16);
        self
    }

    pub fn water_layers(mut self, layers: u32) -> Self {
        self.config.water_layers = layers.clamp(1, 4);
        self
    }

    pub fn wave_height(mut self, height: f32) -> Self {
        self.config.wave_height = height.clamp(0.0, 2.0);
        self
    }

    pub fn wave_speed(mut self, speed: f32) -> Self {
        self.config.wave_speed = speed.clamp(0.0, 5.0);
        self
    }

    pub fn enable_animation(mut self, enabled: bool) -> Self {
        self.config.enable_animation = enabled;
        self
    }

    pub fn build(self) -> OceanConfig {
        self.config
    }
}
