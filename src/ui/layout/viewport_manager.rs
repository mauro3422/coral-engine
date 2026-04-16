// Viewport Manager - Coral Engine
// Manages multiple viewports with different views

use super::node_types::ViewportType;
use std::collections::HashMap;

pub struct CameraState {
    pub position: [f32; 3],
    pub rotation: [f32; 2], // yaw, pitch
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            position: [0.0, 5.0, 10.0],
            rotation: [-45.0, -25.0],
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Clone for CameraState {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            rotation: self.rotation,
            fov: self.fov,
            near: self.near,
            far: self.far,
        }
    }
}

impl CameraState {
    pub fn top_down() -> Self {
        Self {
            position: [0.0, 20.0, 0.0],
            rotation: [0.0, -90.0],
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }

    pub fn front() -> Self {
        Self {
            position: [0.0, 0.0, 15.0],
            rotation: [0.0, 0.0],
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }

    pub fn side() -> Self {
        Self {
            position: [15.0, 0.0, 0.0],
            rotation: [90.0, 0.0],
            fov: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }

    pub fn perspective() -> Self {
        Self::default()
    }

    pub fn for_viewport_type(vt: ViewportType) -> Self {
        match vt {
            ViewportType::Perspective3D => Self::perspective(),
            ViewportType::TopDown => Self::top_down(),
            ViewportType::Front => Self::front(),
            ViewportType::Side => Self::side(),
            ViewportType::UVEditor | ViewportType::GraphEditor => Self::front(),
            ViewportType::Camera => Self::perspective(),
        }
    }
}

#[derive(Clone)]
pub struct ViewportState {
    pub id: super::node_types::ViewportId,
    pub viewport_type: ViewportType,
    pub camera: CameraState,
    pub show_grid: bool,
    pub show_axes: bool,
    pub background_color: [f32; 3],
    pub render_target: Option<super::node_types::ViewportId>,
}

pub struct ViewportManager {
    viewports: HashMap<super::node_types::ViewportId, ViewportState>,
    active: super::node_types::ViewportId,
    next_id: super::node_types::ViewportId,
}

impl Default for ViewportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewportManager {
    pub fn new() -> Self {
        let mut viewports = HashMap::new();

        // Create main viewport
        let main = ViewportState {
            id: 1,
            viewport_type: ViewportType::Perspective3D,
            camera: CameraState::default(),
            show_grid: true,
            show_axes: true,
            background_color: [0.4, 0.6, 0.9],
            render_target: None,
        };

        viewports.insert(1, main);

        Self {
            viewports,
            active: 1,
            next_id: 2,
        }
    }

    pub fn create_viewport(
        &mut self,
        viewport_type: ViewportType,
    ) -> super::node_types::ViewportId {
        let id = self.next_id;
        self.next_id += 1;

        let viewport = ViewportState {
            id,
            viewport_type,
            camera: CameraState::for_viewport_type(viewport_type),
            show_grid: true,
            show_axes: true,
            background_color: [0.4, 0.6, 0.9],
            render_target: None,
        };

        self.viewports.insert(id, viewport);
        id
    }

    pub fn set_active(&mut self, id: super::node_types::ViewportId) {
        if self.viewports.contains_key(&id) {
            self.active = id;
        }
    }

    pub fn close_viewport(&mut self, id: super::node_types::ViewportId) {
        if id != 1 {
            self.viewports.remove(&id);
            if self.active == id {
                self.active = 1;
            }
        }
    }

    pub fn get_active(&self) -> &ViewportState {
        self.viewports.get(&self.active).unwrap()
    }

    pub fn get_active_mut(&mut self) -> &mut ViewportState {
        self.viewports.get_mut(&self.active).unwrap()
    }

    pub fn get(&self, id: super::node_types::ViewportId) -> Option<&ViewportState> {
        self.viewports.get(&id)
    }

    pub fn get_mut(&mut self, id: super::node_types::ViewportId) -> Option<&mut ViewportState> {
        self.viewports.get_mut(&id)
    }

    pub fn get_active_id(&self) -> super::node_types::ViewportId {
        self.active
    }

    pub fn count(&self) -> usize {
        self.viewports.len()
    }

    // === Camera Operations ===

    pub fn sync_cameras(
        &mut self,
        source: super::node_types::ViewportId,
        target: super::node_types::ViewportId,
    ) {
        let camera = self.viewports.get(&source).map(|v| v.camera.clone());
        if let Some(cam) = camera {
            if let Some(tgt) = self.viewports.get_mut(&target) {
                tgt.camera = cam;
            }
        }
    }

    pub fn sync_to_all(&mut self, source: super::node_types::ViewportId) {
        let camera = if let Some(src) = self.viewports.get(&source) {
            Some(src.camera.clone())
        } else {
            None
        };

        if let Some(cam) = camera {
            for (id, vp) in &mut self.viewports {
                if *id != source {
                    vp.camera = cam.clone();
                }
            }
        }
    }

    pub fn set_viewport_type(&mut self, id: super::node_types::ViewportId, vt: ViewportType) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.viewport_type = vt;
            vp.camera = CameraState::for_viewport_type(vt);
        }
    }

    pub fn toggle_grid(&mut self, id: super::node_types::ViewportId) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.show_grid = !vp.show_grid;
        }
    }

    pub fn toggle_axes(&mut self, id: super::node_types::ViewportId) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.show_axes = !vp.show_axes;
        }
    }

    pub fn set_background(&mut self, id: super::node_types::ViewportId, color: [f32; 3]) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.background_color = color;
        }
    }

    // === Camera Movement (called from input) ===

    pub fn move_camera(
        &mut self,
        id: super::node_types::ViewportId,
        forward: f32,
        right: f32,
        up: f32,
    ) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            let yaw = vp.camera.rotation[0].to_radians();

            vp.camera.position[0] += forward * yaw.sin() + right * yaw.cos();
            vp.camera.position[2] += forward * yaw.cos() - right * yaw.sin();
            vp.camera.position[1] += up;
        }
    }

    pub fn rotate_camera(&mut self, id: super::node_types::ViewportId, yaw: f32, pitch: f32) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.camera.rotation[0] += yaw;
            vp.camera.rotation[1] = (vp.camera.rotation[1] + pitch).clamp(-89.0, 89.0);
        }
    }

    pub fn zoom_camera(&mut self, id: super::node_types::ViewportId, delta: f32) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.camera.fov = (vp.camera.fov + delta).clamp(20.0, 120.0);
        }
    }

    pub fn reset_camera(&mut self, id: super::node_types::ViewportId) {
        if let Some(vp) = self.viewports.get_mut(&id) {
            vp.camera = CameraState::for_viewport_type(vp.viewport_type);
        }
    }

    // === Viewport Iteration ===

    pub fn all_viewports(
        &self,
    ) -> impl Iterator<Item = (&super::node_types::ViewportId, &ViewportState)> {
        self.viewports.iter()
    }

    pub fn all_viewports_mut(
        &mut self,
    ) -> impl Iterator<Item = (&super::node_types::ViewportId, &mut ViewportState)> {
        self.viewports.iter_mut()
    }
}
