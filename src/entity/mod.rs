// Entity System - Coral Engine
// Basic entity framework for NPCs and interactive objects

use crate::physics::{PhysicsBody, AABB};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Npc,
    Item,
    Projectile,
    Trigger,
    Static,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityState {
    Idle,
    Walking,
    Running,
    Jumping,
    Attacking,
    Dead,
    Sleeping,
    Talking,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub entity_type: EntityType,
    pub state: EntityState,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub physics: Option<PhysicsBody>,
    pub health: f32,
    pub max_health: f32,
    pub visible: bool,
    pub collidable: bool,
}

pub type EntityId = u32;

impl Entity {
    pub fn new(id: EntityId, name: String, entity_type: EntityType) -> Self {
        Self {
            id,
            name,
            entity_type,
            state: EntityState::Idle,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            physics: None,
            health: 100.0,
            max_health: 100.0,
            visible: true,
            collidable: true,
        }
    }

    pub fn with_position(mut self, pos: [f32; 3]) -> Self {
        self.position = pos;
        self
    }

    pub fn with_physics(mut self, size: [f32; 3]) -> Self {
        self.physics = Some(PhysicsBody::new(self.position, size));
        self
    }

    pub fn aabb(&self) -> AABB {
        AABB::from_center_and_size(self.position, self.scale)
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.health = (self.health - amount).max(0.0);
        if self.health <= 0.0 {
            self.state = EntityState::Dead;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn distance_to(&self, other: &Entity) -> f32 {
        let dx = self.position[0] - other.position[0];
        let dy = self.position[1] - other.position[1];
        let dz = self.position[2] - other.position[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn is_alive(&self) -> bool {
        self.state != EntityState::Dead && self.health > 0.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct EntityManager {
    entities: Vec<Entity>,
    next_id: EntityId,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 1,
        }
    }

    pub fn spawn(&mut self, name: String, entity_type: EntityType, position: [f32; 3]) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity::new(id, name, entity_type).with_position(position);
        self.entities.push(entity);
        id
    }

    pub fn spawn_with_physics(
        &mut self,
        name: String,
        entity_type: EntityType,
        position: [f32; 3],
        size: [f32; 3],
    ) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;

        let entity = Entity::new(id, name, entity_type)
            .with_position(position)
            .with_physics(size);
        self.entities.push(entity);
        id
    }

    pub fn despawn(&mut self, id: EntityId) -> bool {
        if let Some(pos) = self.entities.iter().position(|e| e.id == id) {
            self.entities.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get(&self, id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == id)
    }

    pub fn update(&mut self, dt: f32) {
        for entity in &mut self.entities {
            if let Some(ref mut physics) = entity.physics {
                physics.update(dt, 9.81);
                entity.position = physics.position;
            }
        }
    }

    pub fn all(&self) -> &[Entity] {
        &self.entities
    }

    pub fn by_type(&self, entity_type: EntityType) -> Vec<&Entity> {
        self.entities
            .iter()
            .filter(|e| e.entity_type == entity_type)
            .collect()
    }

    pub fn count(&self) -> usize {
        self.entities.len()
    }

    pub fn within_distance(&self, position: [f32; 3], radius: f32) -> Vec<&Entity> {
        self.entities
            .iter()
            .filter(|e| {
                let dx = e.position[0] - position[0];
                let dy = e.position[1] - position[1];
                let dz = e.position[2] - position[2];
                (dx * dx + dy * dy + dz * dz).sqrt() <= radius
            })
            .collect()
    }
}
