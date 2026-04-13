// Ocean configuration - canonical definition
// Provides builder pattern with validation for ocean simulation parameters

pub const DEFAULT_VOXEL_SIZE: f32 = 0.5;
pub const MIN_VOXEL_SIZE: f32 = 0.1;
pub const MAX_VOXEL_SIZE: f32 = 2.0;

pub const DEFAULT_BLOCK_WORLD_SIZE: f32 = 4.0;
pub const DEFAULT_BLOCK_COUNT_X: i32 = 1;
pub const DEFAULT_BLOCK_COUNT_Z: i32 = 1;
pub const DEFAULT_WAVE_HEIGHT: f32 = 0.3;
pub const DEFAULT_WAVE_SPEED: f32 = 1.0;
pub const DEFAULT_WATER_LAYERS: u32 = 4;

#[derive(Clone, Debug)]
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
            water_color: [0.1, 0.35, 0.7],
            enable_animation: true,
        };
        cfg.validate();
        cfg
    }

    pub fn recalculate_block_size(&mut self) {
        self.block_size = (self.block_world_size / self.voxel_size).round() as u32;
        self.block_size = self.block_size.clamp(2, 64);
    }

    pub fn validate(&mut self) {
        self.voxel_size = self.voxel_size.clamp(MIN_VOXEL_SIZE, MAX_VOXEL_SIZE);
        self.block_world_size = self.block_world_size.clamp(1.0, 16.0);
        self.recalculate_block_size();
        self.block_count_x = self.block_count_x.clamp(1, 16);
        self.block_count_z = self.block_count_z.clamp(1, 16);
        self.water_layers = self.water_layers.clamp(1, 4);
        self.wave_height = self.wave_height.clamp(0.0, 2.0);
        self.wave_speed = self.wave_speed.clamp(0.0, 5.0);
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
        Self { config: OceanConfig::default() }
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

    pub fn build(mut self) -> OceanConfig {
        self.config.validate();
        self.config
    }
}
