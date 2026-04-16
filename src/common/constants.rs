// Constants - Coral Engine
// Centralized constants for all subsystems

// ============================================================================
// Camera
// ============================================================================

pub const DEFAULT_CAMERA_POSITION: [f32; 3] = [20.0, 20.0, 60.0];
pub const DEFAULT_CAMERA_PITCH: f32 = -25.0;
pub const DEFAULT_CAMERA_YAW: f32 = 180.0;
pub const DEFAULT_CAMERA_FOV: f32 = 60.0;
pub const DEFAULT_CAMERA_SPEED: f32 = 15.0;
pub const DEFAULT_CAMERA_SENSITIVITY: f32 = 0.15;

pub const MIN_CAMERA_PITCH: f32 = -89.0;
pub const MAX_CAMERA_PITCH: f32 = 89.0;
pub const MIN_CAMERA_FOV: f32 = 30.0;
pub const MAX_CAMERA_FOV: f32 = 120.0;
pub const NEAR_PLANE: f32 = 0.1;
pub const FAR_PLANE: f32 = 500.0;

// ============================================================================
// Camera Movement
// ============================================================================

pub const CAMERA_SPEED_BOOST_MULT: f32 = 2.0;
pub const CAMERA_SPEED_SLOW_MULT: f32 = 0.33;
pub const CAMERA_ZOOM_SPEED: f32 = 5.0;

// ============================================================================
// Input
// ============================================================================

pub const MOUSE_SENSITIVITY: f32 = 0.15;
pub const MOUSE_CAPTURE_THRESHOLD: f64 = 1.0;

// ============================================================================
// Ocean
// ============================================================================

pub const DEFAULT_VOXEL_SIZE: f32 = 0.5;
pub const MIN_VOXEL_SIZE: f32 = 0.1;
pub const MAX_VOXEL_SIZE: f32 = 2.0;

pub const DEFAULT_BLOCK_WORLD_SIZE: f32 = 4.0;
pub const MIN_BLOCK_WORLD_SIZE: f32 = 1.0;
pub const MAX_BLOCK_WORLD_SIZE: f32 = 16.0;

pub const DEFAULT_BLOCK_SIZE: u32 = 8;
pub const MIN_BLOCK_SIZE: u32 = 2;
pub const MAX_BLOCK_SIZE: u32 = 64;

pub const DEFAULT_BLOCK_COUNT_X: i32 = 1;
pub const DEFAULT_BLOCK_COUNT_Z: i32 = 1;
pub const MAX_BLOCK_COUNT: i32 = 16;

pub const DEFAULT_WATER_LAYERS: u32 = 4;
pub const MIN_WATER_LAYERS: u32 = 1;
pub const MAX_WATER_LAYERS: u32 = 4;

pub const DEFAULT_WAVE_HEIGHT: f32 = 0.3;
pub const DEFAULT_WAVE_SPEED: f32 = 1.0;
pub const MAX_WAVE_HEIGHT: f32 = 2.0;
pub const MAX_WAVE_SPEED: f32 = 5.0;

pub const DEFAULT_WATER_COLOR: [f32; 3] = [0.1, 0.35, 0.7];

// ============================================================================
// Terrain
// ============================================================================

pub const DEFAULT_TERRAIN_SIZE_X: u32 = 64;
pub const DEFAULT_TERRAIN_SIZE_Y: u32 = 32;
pub const DEFAULT_TERRAIN_SIZE_Z: u32 = 64;
pub const DEFAULT_TERRAIN_VOXEL_SIZE: f32 = 1.0;
pub const DEFAULT_TERRAIN_SEA_LEVEL: f32 = 8.0;
pub const DEFAULT_TERRAIN_HEIGHT: f32 = 16.0;
pub const DEFAULT_TERRAIN_NOISE_SCALE: f32 = 0.05;
pub const DEFAULT_TERRAIN_SEED: u32 = 12345;

// ============================================================================
// Physics
// ============================================================================

pub const DEFAULT_GRAVITY: f32 = 9.81;
pub const DEFAULT_ENTITY_SIZE: [f32; 3] = [1.0, 1.8, 1.0];
pub const DEFAULT_ENTITY_MASS: f32 = 1.0;
pub const DEFAULT_COLLISION_MARGIN: f32 = 0.01;

// ============================================================================
// Rendering
// ============================================================================

pub const DEFAULT_WINDOW_WIDTH: u32 = 1280;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 720;
pub const DEFAULT_BACKGROUND_COLOR: wgpu::Color = wgpu::Color {
    r: 0.4,
    g: 0.6,
    b: 0.9,
    a: 1.0,
};

// ============================================================================
// Timing
// ============================================================================

pub const MAX_DELTA_TIME: f32 = 0.05;
pub const FPS_UPDATE_INTERVAL: f64 = 0.5;
pub const AUTO_SAVE_INTERVAL: f64 = 60.0;

// ============================================================================
// UI
// ============================================================================

pub const DEFAULT_PANEL_WIDTH: f32 = 280.0;
