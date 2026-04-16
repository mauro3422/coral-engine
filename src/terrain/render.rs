//! Terrain Render - Coral Engine
//! Convert terrain voxels to renderable geometry

use crate::common::ColoredFace;
#[allow(unused_imports)]
use crate::terrain::TerrainWorld;

pub type TerrainFace = ColoredFace;

pub struct TerrainRender {
    pub faces: Vec<TerrainFace>,
}

impl TerrainRender {
    pub fn new() -> Self {
        Self { faces: Vec::new() }
    }

    pub fn build_from_terrain(&mut self, terrain: &TerrainWorld) {
        self.faces.clear();
        let config = &terrain.config;
        let (sx, sy, sz) = (config.size_x, config.size_y, config.size_z);
        let vs = config.voxel_size;

        for z in 0..sz {
            for y in 0..sy {
                for x in 0..sx {
                    if let Some(voxel) = terrain.get_voxel(x, y, z) {
                        if !voxel.block.is_solid() {
                            continue;
                        }

                        let px = x as f32 * vs;
                        let py = y as f32 * vs;
                        let pz = z as f32 * vs;

                        // Check each face - only add if exposed
                        // Top
                        if !terrain.is_solid(x, y + 1, z) {
                            self.faces.push(TerrainFace {
                                position: [px, py + vs, pz],
                                normal: [0.0, 1.0, 0.0],
                                color: voxel.block.color(),
                            });
                        }
                        // Bottom
                        if !terrain.is_solid(x, y.saturating_sub(1), z) {
                            let base_color = voxel.block.color();
                            self.faces.push(TerrainFace {
                                position: [px, py, pz],
                                normal: [0.0, -1.0, 0.0],
                                color: [
                                    base_color[0] * 0.7,
                                    base_color[1] * 0.7,
                                    base_color[2] * 0.7,
                                ],
                            });
                        }
                        // Front
                        if !terrain.is_solid(x, y, z + 1) {
                            let base_color = voxel.block.color();
                            self.faces.push(TerrainFace {
                                position: [px, py, pz + vs],
                                normal: [0.0, 0.0, 1.0],
                                color: [
                                    base_color[0] * 0.8,
                                    base_color[1] * 0.8,
                                    base_color[2] * 0.8,
                                ],
                            });
                        }
                        // Back
                        if !terrain.is_solid(x, y, z.saturating_sub(1)) {
                            let base_color = voxel.block.color();
                            self.faces.push(TerrainFace {
                                position: [px, py, pz],
                                normal: [0.0, 0.0, -1.0],
                                color: [
                                    base_color[0] * 0.8,
                                    base_color[1] * 0.8,
                                    base_color[2] * 0.8,
                                ],
                            });
                        }
                        // Right
                        if !terrain.is_solid(x + 1, y, z) {
                            let base_color = voxel.block.color();
                            self.faces.push(TerrainFace {
                                position: [px + vs, py, pz],
                                normal: [1.0, 0.0, 0.0],
                                color: [
                                    base_color[0] * 0.9,
                                    base_color[1] * 0.9,
                                    base_color[2] * 0.9,
                                ],
                            });
                        }
                        // Left
                        if !terrain.is_solid(x.saturating_sub(1), y, z) {
                            let base_color = voxel.block.color();
                            self.faces.push(TerrainFace {
                                position: [px, py, pz],
                                normal: [-1.0, 0.0, 0.0],
                                color: [
                                    base_color[0] * 0.9,
                                    base_color[1] * 0.9,
                                    base_color[2] * 0.9,
                                ],
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn clear(&mut self) {
        self.faces.clear();
    }
}

impl Default for TerrainRender {
    fn default() -> Self {
        Self::new()
    }
}
