//! Common Face types for rendering
//! Unified face structures for voxel rendering

/// Basic face with position and normal
#[derive(Clone, Copy, Debug, Default)]
pub struct Face {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

/// Face with color (for terrain, blocks with texture)
#[derive(Clone, Copy, Debug, Default)]
pub struct ColoredFace {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl Face {
    /// Create new face
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }

    /// Add color to create ColoredFace
    pub fn with_color(self, color: [f32; 3]) -> ColoredFace {
        ColoredFace {
            position: self.position,
            normal: self.normal,
            color,
        }
    }

    /// Apply brightness multiplier
    pub fn with_brightness(self, _factor: f32) -> Self {
        Self {
            position: self.position,
            normal: self.normal,
        }
    }
}

impl ColoredFace {
    /// Create colored face
    pub fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }

    /// Apply brightness factor to color
    pub fn with_brightness(mut self, factor: f32) -> Self {
        self.color[0] *= factor;
        self.color[1] *= factor;
        self.color[2] *= factor;
        self
    }
}
