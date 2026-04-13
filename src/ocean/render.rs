// Ocean render types - canonical definitions for renderer consumption

/// A single visible face of a water voxel
#[derive(Clone, Copy, Debug)]
pub struct WaterFace {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

/// Canonical block type constants - single source of truth
pub mod block_types {
    pub const AIR: u8 = 0;
    pub const WATER: u8 = 7;
}
