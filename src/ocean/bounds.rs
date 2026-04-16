// Bounding box for any object in the world
// Standard API so wireframes, collision, and LOD stay in sync automatically

/// Standard dimensions for any world object
/// Implement this trait on any object to get automatic wireframe/collision sync
#[derive(Clone, Copy, Debug)]
pub struct ObjectBounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl ObjectBounds {
    pub fn from_center_and_half_size(center: [f32; 3], half: [f32; 3]) -> Self {
        Self {
            min: [
                center[0] - half[0],
                center[1] - half[1],
                center[2] - half[2],
            ],
            max: [
                center[0] + half[0],
                center[1] + half[1],
                center[2] + half[2],
            ],
        }
    }

    pub fn from_origin_and_size(origin: [f32; 3], size: [f32; 3]) -> Self {
        Self {
            min: origin,
            max: [
                origin[0] + size[0],
                origin[1] + size[1],
                origin[2] + size[2],
            ],
        }
    }

    pub fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) / 2.0,
            (self.min[1] + self.max[1]) / 2.0,
            (self.min[2] + self.max[2]) / 2.0,
        ]
    }

    pub fn size(&self) -> [f32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    pub fn width(&self) -> f32 {
        self.max[0] - self.min[0]
    }
    pub fn height(&self) -> f32 {
        self.max[1] - self.min[1]
    }
    pub fn depth(&self) -> f32 {
        self.max[2] - self.min[2]
    }
}
