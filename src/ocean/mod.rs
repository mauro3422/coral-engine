//! Ocean module - Water simulation system
//!
//! # Public API
//! - [`OceanConfig`] - Configuration with builder pattern
//! - [`OceanWorld`] - Main world simulation
//! - [`OceanDimensions`] - Spatial dimensions derived from config
//! - [`ObjectBounds`] - Universal bounding box for objects
//! - [`WaterFace`] - Renderable face
//! - [`BlockRegistry`] - Block metadata management

pub mod bounds;
pub mod config;
pub mod registry;
pub mod render;
pub mod world;

mod block; // Internal

// Re-exports
pub use block::BlockPos;
pub use bounds::ObjectBounds;
pub use config::OceanConfig;
pub use registry::{BlockMetadata, BlockRegistry, BlockTag};
pub use world::{OceanDimensions, OceanWorld};
