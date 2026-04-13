// WaterBlock - A single block of water voxels (internal, not public API)

use crate::ocean::render::WaterFace;

pub const AIR: u8 = 0;
pub const WATER: u8 = 7;
pub const SAND: u8 = 2;

#[derive(Clone, Copy, Debug)]
pub struct VoxelData {
    pub block_type: u8,
}

impl VoxelData {
    pub const fn water() -> Self { Self { block_type: WATER } }
    pub const fn sand() -> Self { Self { block_type: SAND } }
    pub const fn air() -> Self { Self { block_type: AIR } }
    pub const fn is_water(&self) -> bool { self.block_type == WATER }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BlockPos {
    pub x: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, z: i32) -> Self { Self { x, z } }
}

#[derive(Clone, Debug)]
pub struct WaterBlock {
    pub position: BlockPos,
    pub voxels: Vec<VoxelData>,
    pub size: u32,
    pub voxel_size: f32,
    pub dirty: bool,
    pub wave_amplitude: f32,
}

impl WaterBlock {
    pub fn new(pos: BlockPos, size: u32, voxel_size: f32) -> Self {
        let total = (size * size * size) as usize;
        Self {
            position: pos,
            voxels: vec![VoxelData::air(); total],
            size,
            voxel_size,
            dirty: false,
            wave_amplitude: 0.3,
        }
    }

    #[inline]
    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        ((y * self.size + z) * self.size + x) as usize
    }

    pub fn get(&self, x: u32, y: u32, z: u32) -> Option<&VoxelData> {
        if x < self.size && y < self.size && z < self.size {
            Some(&self.voxels[self.idx(x, y, z)])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: u32, y: u32, z: u32, data: VoxelData) {
        if x < self.size && y < self.size && z < self.size {
            let idx = self.idx(x, y, z);
            self.voxels[idx] = data;
            self.dirty = true;
        }
    }

    pub fn fill_with_surface_water(&mut self, water_layers: u32) {
        let total_layers = self.size;
        let water_end = water_layers.min(total_layers);
        for ly in 0..water_end {
            for lz in 0..self.size {
                for lx in 0..self.size {
                    self.set(lx, ly, lz, VoxelData::water());
                }
            }
        }
        self.dirty = true;
    }

    pub fn collect_visible_faces(&self, time: f32, animated: bool) -> Vec<WaterFace> {
        let mut faces = Vec::new();
        let vs = self.voxel_size;

        for ly in 0..self.size {
            for lz in 0..self.size {
                for lx in 0..self.size {
                    let Some(voxel) = self.get(lx, ly, lz) else { continue };
                    if !voxel.is_water() { continue }

                    let block_offset_x = (self.position.x * self.size as i32) as f32 * vs;
                    let block_offset_z = (self.position.z * self.size as i32) as f32 * vs;

                    let wx = block_offset_x + (lx as f32 * vs) - (self.size as f32 * vs) / 2.0;
                    let wy = (ly as f32 * vs) - (self.size as f32 * vs) / 2.0;
                    let wz = block_offset_z + (lz as f32 * vs) - (self.size as f32 * vs) / 2.0;

                    let wave_offset = if animated {
                        (time * 2.0f32 + wx * 0.5f32 + wz * 0.3f32).sin() * self.wave_amplitude
                    } else {
                        0.0
                    };
                    let final_y = wy + wave_offset;

                    self.add_face_if_exposed(&mut faces, wx, final_y, wz, lx, ly, lz, 0, 0, 1);
                    self.add_face_if_exposed(&mut faces, wx, final_y, wz, lx, ly, lz, 0, 0, -1);
                    self.add_face_if_exposed(&mut faces, wx, final_y, wz, lx, ly, lz, 0, 1, 0);
                    self.add_face_if_exposed(&mut faces, wx, final_y, wz, lx, ly, lz, 0, -1, 0);
                    self.add_face_if_exposed(&mut faces, wx, final_y, wz, lx, ly, lz, 1, 0, 0);
                    self.add_face_if_exposed(&mut faces, wx, final_y, wz, lx, ly, lz, -1, 0, 0);
                }
            }
        }
        faces
    }

    fn add_face_if_exposed(
        &self,
        faces: &mut Vec<WaterFace>,
        wx: f32, wy: f32, wz: f32,
        lx: u32, ly: u32, lz: u32,
        nx: i32, ny: i32, nz: i32,
    ) {
        let check_x = lx.wrapping_add(nx as u32);
        let check_y = ly.wrapping_add(ny as u32);
        let check_z = lz.wrapping_add(nz as u32);

        let exposed = if let Some(neighbor) = self.get(check_x, check_y, check_z) {
            !neighbor.is_water()
        } else {
            true
        };

        if exposed {
            faces.push(WaterFace {
                position: [wx, wy, wz],
                normal: [nx as f32, ny as f32, nz as f32],
            });
        }
    }
}
