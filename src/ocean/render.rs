//! Ocean render types
//! Canonical definitions for renderer consumption

use crate::common::ColoredFace;

/// Water face - simplified without per-face color (water is uniform)
#[derive(Clone, Copy, Debug)]
pub struct WaterFace {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl WaterFace {
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }

    /// Convert to ColoredFace with water color
    pub fn to_colored(self) -> ColoredFace {
        ColoredFace {
            position: self.position,
            normal: self.normal,
            color: [0.1, 0.35, 0.7], // Water color
        }
    }
}
