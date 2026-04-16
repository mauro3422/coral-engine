//! Common Types - Coral Engine
//! Fundamental traits and types for all subsystems
//!
//! This module provides:
//! - Core traits for world management (`World`, `Renderable`, `Dimensions`)
//! - Configuration traits (`Configurable`, `ConfigMut`)
//! - Centralized error types (`Error`, `Result`)
//! - Shared constants for all subsystems
//! - Common types like `Face`, `ColoredFace`

pub mod blocks;
pub mod constants;
pub mod face;

// Core types - used by other modules
pub use blocks::block_types;
#[allow(unused_imports)]
pub use blocks::validation;
pub use constants::*;
pub use face::ColoredFace;

#[allow(unused_imports)]
const _: () = (); // Placeholder for module activation

// ============================================================================
// Core Traits
// ============================================================================

/// Trait for any simulation world (ocean, terrain, etc.)
///
/// Implementors must provide:
/// - `update()` for frame-by-frame simulation
/// - `generate()` for world creation/regeneration
/// - `dimensions()` for spatial information
pub trait World {
    /// Update world state for one frame
    fn update(&mut self, dt: f32);
    /// Generate or regenerate world content
    fn generate(&mut self);
    /// Get world dimensions/bounds
    fn dimensions(&self) -> Box<dyn Dimensions + '_>;
}

/// Trait for voxel-based worlds
pub trait VoxelWorld {
    /// Get total voxel count
    fn total_voxels(&self) -> usize;
    /// Get active/solid voxel count
    fn active_voxels(&self) -> usize;
    /// Check if voxel at position is solid
    fn is_solid(&self, x: i32, y: i32, z: i32) -> bool;
}

/// Trait for objects with spatial dimensions
pub trait Dimensions: Send {
    /// Width along X axis
    fn width(&self) -> f32;
    /// Height along Y axis
    fn height(&self) -> f32;
    /// Depth along Z axis
    fn depth(&self) -> f32;
    /// Get bounding box for collision/selection
    fn bounds(&self) -> crate::ocean::ObjectBounds;
}

/// Trait for renderable world content
pub trait Renderable {
    /// Type of face/primitive to render
    type Face;
    /// Get visible faces for rendering
    fn visible_faces(&self) -> Vec<Self::Face>;
    /// Total potential voxels in world
    fn total_voxels(&self) -> usize;
    /// Currently active/visible voxels
    fn active_voxels(&self) -> usize;
}

/// Trait for objects with configuration
pub trait Configurable {
    type Config;
    /// Get immutable reference to config
    fn config(&self) -> &Self::Config;
    /// Get mutable reference to config
    fn config_mut(&mut self) -> &mut Self::Config;
}

/// Mutable configuration operations for ocean-like configs
pub trait ConfigMut {
    fn set_voxel_size(&mut self, size: f32);
    fn set_block_world_size(&mut self, size: f32);
    fn set_water_layers(&mut self, layers: u32);
    fn set_wave_height(&mut self, height: f32);
    fn set_wave_speed(&mut self, speed: f32);
    fn set_enable_animation(&mut self, enabled: bool);
}

// ============================================================================
// Result Types
// ============================================================================

/// Alias for engine results with custom error type
pub type Result<T> = std::result::Result<T, Error>;

/// Engine-specific error types
#[derive(Debug, Clone)]
pub enum Error {
    /// Resource not found
    NotFound(String),
    /// Invalid configuration value
    InvalidConfig(String),
    /// Rendering error
    Render(String),
    /// Input handling error
    Input(String),
    /// Physics simulation error
    Physics(String),
    /// I/O operation error
    Io(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(s) => write!(f, "Not found: {}", s),
            Error::InvalidConfig(s) => write!(f, "Invalid config: {}", s),
            Error::Render(s) => write!(f, "Render error: {}", s),
            Error::Input(s) => write!(f, "Input error: {}", s),
            Error::Physics(s) => write!(f, "Physics error: {}", s),
            Error::Io(s) => write!(f, "IO error: {}", s),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create a NotFound error
    pub fn not_found(entity: &str) -> Self {
        Error::NotFound(entity.to_string())
    }

    /// Create an InvalidConfig error
    pub fn invalid_config(field: &str, reason: &str) -> Self {
        Error::InvalidConfig(format!("{}: {}", field, reason))
    }

    /// Create a Render error
    pub fn render(msg: &str) -> Self {
        Error::Render(msg.to_string())
    }

    /// Create an Input error
    pub fn input(msg: &str) -> Self {
        Error::Input(msg.to_string())
    }
}

// ============================================================================
// Factory Traits
// ============================================================================

/// Factory for creating world instances with configuration
pub trait WorldFactory {
    type Config;
    type World;

    /// Create world from config
    fn create(config: Self::Config) -> Self::World;
    /// Get default config
    fn default_config() -> Self::Config;
}

/// Builder for creating complex configurations
pub trait ConfigBuilder: Sized {
    type Config;

    fn build(self) -> Self::Config;
    fn default() -> Self::Config;
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get file extension
pub fn file_extension(path: &str) -> Option<&str> {
    std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
}

/// Check if file exists
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// ============================================================================
// Error Handling Utilities
// ============================================================================

/// Convert Option to Result with not found error
pub fn ok_or_not_found<T>(opt: Option<T>, entity: &str) -> Result<T> {
    opt.ok_or_else(|| Error::not_found(entity))
}

/// Convert external Result to engine Result
pub fn convert_err<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> Result<T> {
    result.map_err(|e| Error::Io(e.to_string()))
}

/// Map external error to engine error
impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::Io(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

impl From<wgpu::SurfaceError> for Error {
    fn from(e: wgpu::SurfaceError) -> Self {
        Error::Render(e.to_string())
    }
}
