// Ocean module - Water simulation system
// Public API: OceanConfig, OceanWorld, OceanDimensions, ObjectBounds, WaterFace, block_types

pub mod bounds;
pub mod config;
pub mod render;
pub mod world;

mod block; // Internal

pub use bounds::ObjectBounds;
pub use config::{OceanConfig, OceanConfigBuilder};
pub use world::{OceanWorld, OceanDimensions};
pub use render::WaterFace;
pub use render::block_types;
