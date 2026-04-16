// Scene Object System - Coral Engine
// Any object in the world (water blocks, terrain, entities, lights)
// Supports hierarchical parent/child relationships and component-based behavior

use crate::core::cartesian::CoordinateSystem3D;
use cgmath::{Matrix4, SquareMatrix};

/// Lifecycle phases for scene components
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentPhase {
    Init,
    Update,
    Render,
    Physics,
    Cleanup,
}

/// Core trait for scene object behavior (voxel-first, simple lifecycle)
#[allow(dead_code)]
pub trait VoxelComponent: Send + Sync {
    fn phase(&self) -> ComponentPhase;
    fn update(&mut self, _delta_time: f32) {}
    fn on_attach(&mut self) {}
    fn on_detach(&mut self) {}
}

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

/// Property that can be edited in the inspector - type-safe
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

    pub fn as_float(&self) -> Option<f32> {
        match self {
            SceneProperty::Float(_, v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            SceneProperty::Int(_, v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SceneProperty::Bool(_, v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<[f32; 3]> {
        match self {
            SceneProperty::Vec3(_, v) => Some(*v),
            _ => None,
        }
    }

    pub fn set_float(&mut self, value: f32) -> bool {
        match self {
            SceneProperty::Float(_, v) => {
                *v = value;
                true
            }
            _ => false,
        }
    }

    pub fn set_int(&mut self, value: i32) -> bool {
        match self {
            SceneProperty::Int(_, v) => {
                *v = value;
                true
            }
            _ => false,
        }
    }

    pub fn set_bool(&mut self, value: bool) -> bool {
        match self {
            SceneProperty::Bool(_, v) => {
                *v = value;
                true
            }
            _ => false,
        }
    }

    pub fn set_vec3(&mut self, value: [f32; 3]) -> bool {
        match self {
            SceneProperty::Vec3(_, v) => {
                *v = value;
                true
            }
            _ => false,
        }
    }
}

/// Any object in the scene graph - hierarchical with parent/child
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
    pub parent_id: Option<ObjectId>,
    pub child_ids: Vec<ObjectId>,
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
            parent_id: None,
            child_ids: Vec::new(),
        }
    }

    pub fn at_position(mut self, pos: [f32; 3]) -> Self {
        self.position = pos;
        self
    }
}

// Debug/render objects (grid, axes, wireframes) - kept separate from world objects
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DebugMesh {
    Grid,
    Axes,
    Wireframe, // Dynamic wireframe for selected object
}

#[derive(Clone, Debug)]
pub struct DebugObject {
    pub mesh: DebugMesh,
    pub transform: Matrix4<f32>,
    pub visible: bool,
}

impl DebugObject {
    pub fn new(mesh: DebugMesh) -> Self {
        Self {
            mesh,
            transform: Matrix4::identity(),
            visible: true,
        }
    }

    pub fn with_transform(mesh: DebugMesh, transform: Matrix4<f32>) -> Self {
        Self {
            mesh,
            transform,
            visible: true,
        }
    }
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
        scene.debug_objects.push(DebugObject::new(DebugMesh::Grid));
        scene.debug_objects.push(DebugObject::new(DebugMesh::Axes));
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

    /// Add child to object (establish parent-child relationship)
    pub fn add_child(&mut self, parent_id: ObjectId, child_id: ObjectId) -> bool {
        if let Some(parent) = self.objects.iter_mut().find(|o| o.id == parent_id) {
            if !parent.child_ids.contains(&child_id) {
                parent.child_ids.push(child_id);
                if let Some(child) = self.objects.iter_mut().find(|o| o.id == child_id) {
                    child.parent_id = Some(parent_id);
                    return true;
                }
            }
        }
        false
    }

    /// Remove child from object
    pub fn remove_child(&mut self, parent_id: ObjectId, child_id: ObjectId) -> bool {
        if let Some(parent) = self.objects.iter_mut().find(|o| o.id == parent_id) {
            if let Some(idx) = parent.child_ids.iter().position(|&id| id == child_id) {
                parent.child_ids.remove(idx);
                if let Some(child) = self.objects.iter_mut().find(|o| o.id == child_id) {
                    child.parent_id = None;
                    return true;
                }
            }
        }
        false
    }

    /// Get children of an object
    pub fn get_children(&self, parent_id: ObjectId) -> Vec<&WorldObject> {
        if let Some(parent) = self.objects.iter().find(|o| o.id == parent_id) {
            parent
                .child_ids
                .iter()
                .filter_map(|&cid| self.objects.iter().find(|o| o.id == cid))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get parent of an object
    pub fn get_parent(&self, id: ObjectId) -> Option<&WorldObject> {
        if let Some(obj) = self.objects.iter().find(|o| o.id == id) {
            obj.parent_id
                .and_then(|pid| self.objects.iter().find(|o| o.id == pid))
        } else {
            None
        }
    }

    pub fn load_from(&mut self, other: Scene) {
        self.objects = other.objects;
        self.debug_objects = other.debug_objects;
        self.next_id = other.next_id;
    }

    /// Get world transform (accumulated from all ancestors)
    /// Returns (world_position, world_rotation, world_scale)
    pub fn get_world_transform(&self, id: ObjectId) -> ([f32; 3], [f32; 3], [f32; 3]) {
        let mut world_pos = [0.0f32, 0.0, 0.0];
        let mut world_rot = [0.0f32, 0.0, 0.0];
        let mut world_scale = [1.0f32, 1.0, 1.0];

        let mut current_id = Some(id);
        while let Some(cid) = current_id {
            if let Some(obj) = self.objects.iter().find(|o| o.id == cid) {
                world_pos[0] += obj.position[0] * world_scale[0];
                world_pos[1] += obj.position[1] * world_scale[1];
                world_pos[2] += obj.position[2] * world_scale[2];
                world_rot[0] += obj.rotation[0];
                world_rot[1] += obj.rotation[1];
                world_rot[2] += obj.rotation[2];
                world_scale[0] *= obj.scale[0];
                world_scale[1] *= obj.scale[1];
                world_scale[2] *= obj.scale[2];
                current_id = obj.parent_id;
            } else {
                break;
            }
        }

        (world_pos, world_rot, world_scale)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::standard()
    }
}
