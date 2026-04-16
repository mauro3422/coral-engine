// Physics System - Coral Engine
// AABB collision detection and basic physics

#[derive(Clone, Copy, Debug, Default)]
pub struct AABB {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl AABB {
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Self { min, max }
    }

    pub fn from_center_and_size(center: [f32; 3], size: [f32; 3]) -> Self {
        let half = [size[0] / 2.0, size[1] / 2.0, size[2] / 2.0];
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

    pub fn intersects(&self, other: &AABB) -> bool {
        self.min[0] <= other.max[0]
            && self.max[0] >= other.min[0]
            && self.min[1] <= other.max[1]
            && self.max[1] >= other.min[1]
            && self.min[2] <= other.max[2]
            && self.max[2] >= other.min[2]
    }

    pub fn contains_point(&self, point: [f32; 3]) -> bool {
        point[0] >= self.min[0]
            && point[0] <= self.max[0]
            && point[1] >= self.min[1]
            && point[1] <= self.max[1]
            && point[2] >= self.min[2]
            && point[2] <= self.max[2]
    }

    pub fn expand(&self, margin: f32) -> Self {
        Self {
            min: [
                self.min[0] - margin,
                self.min[1] - margin,
                self.min[2] - margin,
            ],
            max: [
                self.max[0] + margin,
                self.max[1] + margin,
                self.max[2] + margin,
            ],
        }
    }

    pub fn translate(&mut self, offset: [f32; 3]) {
        self.min[0] += offset[0];
        self.min[1] += offset[1];
        self.min[2] += offset[2];
        self.max[0] += offset[0];
        self.max[1] += offset[1];
        self.max[2] += offset[2];
    }
}

#[derive(Clone, Debug, Default)]
pub struct PhysicsBody {
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub aabb: AABB,
    pub mass: f32,
    pub gravity_enabled: bool,
    pub is_grounded: bool,
}

impl PhysicsBody {
    pub fn new(position: [f32; 3], size: [f32; 3]) -> Self {
        Self {
            position,
            velocity: [0.0, 0.0, 0.0],
            aabb: AABB::from_center_and_size(position, size),
            mass: 1.0,
            gravity_enabled: true,
            is_grounded: false,
        }
    }

    pub fn update(&mut self, dt: f32, gravity: f32) {
        if self.gravity_enabled && !self.is_grounded {
            self.velocity[1] -= gravity * dt;
        }

        self.position[0] += self.velocity[0] * dt;
        self.position[1] += self.velocity[1] * dt;
        self.position[2] += self.velocity[2] * dt;

        self.aabb = AABB::from_center_and_size(self.position, self.aabb.size());
    }

    pub fn apply_force(&mut self, force: [f32; 3]) {
        self.velocity[0] += force[0] / self.mass;
        self.velocity[1] += force[1] / self.mass;
        self.velocity[2] += force[2] / self.mass;
    }

    pub fn set_velocity(&mut self, velocity: [f32; 3]) {
        self.velocity = velocity;
    }

    pub fn stop_horizontal(&mut self) {
        self.velocity[0] = 0.0;
        self.velocity[2] = 0.0;
    }
}

#[derive(Clone, Debug, Default)]
pub struct PhysicsWorld {
    pub bodies: Vec<PhysicsBody>,
    pub static_colliders: Vec<AABB>,
    pub gravity: f32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            static_colliders: Vec::new(),
            gravity: 9.81,
        }
    }

    pub fn add_body(&mut self, body: PhysicsBody) -> usize {
        let id = self.bodies.len();
        self.bodies.push(body);
        id
    }

    pub fn add_static_collider(&mut self, aabb: AABB) {
        self.static_colliders.push(aabb);
    }

    pub fn update(&mut self, dt: f32) {
        for body in &mut self.bodies {
            body.is_grounded = false;
            body.update(dt, self.gravity);

            for collider in &self.static_colliders {
                if body.aabb.intersects(collider) {
                    self.resolve_collision(body, collider);
                }
            }
        }
    }

    fn resolve_collision(&mut self, body: &mut PhysicsBody, collider: &AABB) {
        let overlap_x =
            (body.aabb.max[0] - collider.min[0]).min(collider.max[0] - body.aabb.min[0]);
        let overlap_y =
            (body.aabb.max[1] - collider.min[1]).min(collider.max[1] - body.aabb.min[1]);
        let overlap_z =
            (body.aabb.max[2] - collider.min[2]).min(collider.max[2] - body.aabb.min[2]);

        let min_overlap = overlap_x.min(overlap_y).min(overlap_z);

        if min_overlap == overlap_x {
            if body.position[0] < collider.center()[0] {
                body.position[0] -= overlap_x;
            } else {
                body.position[0] += overlap_x;
            }
            body.velocity[0] = 0.0;
        } else if min_overlap == overlap_z {
            if body.position[2] < collider.center()[2] {
                body.position[2] -= overlap_z;
            } else {
                body.position[2] += overlap_z;
            }
            body.velocity[2] = 0.0;
        } else {
            if body.position[1] < collider.center()[1] {
                body.position[1] -= overlap_y;
                body.is_grounded = true;
            } else {
                body.position[1] += overlap_y;
            }
            body.velocity[1] = 0.0;
        }

        body.aabb = AABB::from_center_and_size(body.position, body.aabb.size());
    }

    pub fn raycast(
        &self,
        origin: [f32; 3],
        direction: [f32; 3],
        max_dist: f32,
    ) -> Option<(usize, f32)> {
        let step_size = 0.1;
        let steps = (max_dist / step_size) as i32;

        for i in 1..=steps {
            let t = i as f32 * step_size;
            let point = [
                origin[0] + direction[0] * t,
                origin[1] + direction[1] * t,
                origin[2] + direction[2] * t,
            ];

            for (idx, collider) in self.static_colliders.iter().enumerate() {
                if collider.contains_point(point) {
                    return Some((idx, t));
                }
            }
        }
        None
    }

    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }

    pub fn collider_count(&self) -> usize {
        self.static_colliders.len()
    }
}
