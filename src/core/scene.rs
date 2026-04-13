// Scene - Simplified for voxel-first architecture
use cgmath::{Matrix4, SquareMatrix};

use crate::core::cartesian::CoordinateSystem3D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneMesh {
    Grid,
    Axes,
}

#[derive(Clone, Debug)]
pub struct SceneObject {
    pub mesh: SceneMesh,
    pub transform: Matrix4<f32>,
}

impl SceneObject {
    pub fn new(mesh: SceneMesh, transform: Matrix4<f32>) -> Self {
        Self { mesh, transform }
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub coordinate_system: CoordinateSystem3D,
    pub objects: Vec<SceneObject>,
}

impl Scene {
    pub fn new(coordinate_system: CoordinateSystem3D) -> Self {
        Self {
            coordinate_system,
            objects: Vec::new(),
        }
    }

    pub fn standard() -> Self {
        Self::new(CoordinateSystem3D::standard())
    }

    pub fn push(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    pub fn push_mesh(&mut self, mesh: SceneMesh, transform: Matrix4<f32>) {
        self.push(SceneObject::new(mesh, transform));
    }

    pub fn push_identity(&mut self, mesh: SceneMesh) {
        self.push_mesh(mesh, Matrix4::identity());
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::standard()
    }
}
