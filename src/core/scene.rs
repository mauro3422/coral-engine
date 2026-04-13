// Scene Object System - Coral Engine
// Any object in the world (water blocks, terrain, entities, lights)

use cgmath::{Matrix4, SquareMatrix};
use crate::core::cartesian::CoordinateSystem3D;

/// Unique ID for scene objects
pub type ObjectId = u64;

/// Type of object in the scene
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldObjectType {
    WaterBlock,
    TerrainBlock,
    Entity,
    Light,
    Custom,
}

impl WorldObjectType {
    pub fn icon(&self) -> &'static str {
        match self {
            WorldObjectType::WaterBlock => "🌊",
            WorldObjectType::TerrainBlock => "🏔️",
            WorldObjectType::Entity => "👤",
            WorldObjectType::Light => "💡",
            WorldObjectType::Custom => "📦",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WorldObjectType::WaterBlock => "Water Block",
            WorldObjectType::TerrainBlock => "Terrain Block",
            WorldObjectType::Entity => "Entity",
            WorldObjectType::Light => "Light",
            WorldObjectType::Custom => "Custom Object",
        }
    }
}

/// Property that can be edited in the inspector
#[derive(Clone, Debug)]
pub enum SceneProperty {
    Float(String, f32),
    Int(String, i32),
    Bool(String, bool),
    String(String, String),
    Vec3(String, [f32; 3]),
}

impl SceneProperty {
    pub fn name(&self) -> &str {
        match self {
            SceneProperty::Float(n, _) => n,
            SceneProperty::Int(n, _) => n,
            SceneProperty::Bool(n, _) => n,
            SceneProperty::String(n, _) => n,
            SceneProperty::Vec3(n, _) => n,
        }
    }
}

/// Any object in the scene graph
#[derive(Clone, Debug)]
pub struct WorldObject {
    pub id: ObjectId,
    pub name: String,
    pub obj_type: WorldObjectType,
    pub visible: bool,
    pub locked: bool,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub properties: Vec<SceneProperty>,
}

impl WorldObject {
    pub fn new(id: ObjectId, name: &str, obj_type: WorldObjectType) -> Self {
        Self {
            id,
            name: name.to_string(),
            obj_type,
            visible: true,
            locked: false,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            properties: Vec::new(),
        }
    }

    pub fn at_position(mut self, pos: [f32; 3]) -> Self {
        self.position = pos;
        self
    }
}

// Debug/render objects (grid, axes) - kept separate from world objects
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugMesh {
    Grid,
    Axes,
}

#[derive(Clone, Debug)]
pub struct DebugObject {
    pub mesh: DebugMesh,
    pub transform: Matrix4<f32>,
}

/// The scene - contains all world objects and debug visuals
#[derive(Clone, Debug)]
pub struct Scene {
    pub coordinate_system: CoordinateSystem3D,
    pub objects: Vec<WorldObject>,
    pub debug_objects: Vec<DebugObject>,
    next_id: ObjectId,
}

impl Scene {
    pub fn new(coordinate_system: CoordinateSystem3D) -> Self {
        Self {
            coordinate_system,
            objects: Vec::new(),
            debug_objects: Vec::new(),
            next_id: 1,
        }
    }

    pub fn standard() -> Self {
        let mut scene = Self::new(CoordinateSystem3D::standard());
        scene.debug_objects.push(DebugObject {
            mesh: DebugMesh::Grid,
            transform: Matrix4::identity(),
        });
        scene.debug_objects.push(DebugObject {
            mesh: DebugMesh::Axes,
            transform: Matrix4::identity(),
        });
        scene
    }

    /// Add a new world object and return its ID
    pub fn add_object(&mut self, name: &str, obj_type: WorldObjectType) -> ObjectId {
        let id = self.next_id;
        self.next_id += 1;
        let obj = WorldObject::new(id, name, obj_type);
        self.objects.push(obj);
        id
    }

    /// Get object by ID
    pub fn get_object(&self, id: ObjectId) -> Option<&WorldObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    /// Get mutable object by ID
    pub fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut WorldObject> {
        self.objects.iter_mut().find(|o| o.id == id)
    }

    /// Remove object by ID
    pub fn remove_object(&mut self, id: ObjectId) -> bool {
        let len = self.objects.len();
        self.objects.retain(|o| o.id != id);
        self.objects.len() < len
    }

    /// Generate next ID without creating object
    pub fn next_id(&self) -> ObjectId {
        self.next_id
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::standard()
    }
}
