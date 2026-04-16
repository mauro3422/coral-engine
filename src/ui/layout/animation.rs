// Panel Animation System - Coral Engine
// Animated transitions for panel resize, collapse, dock

use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationType {
    Resize {
        axis: ResizeAxis,
    },
    Collapse {
        target_collapsed: bool,
    },
    Dock {
        target_region: super::node_types::PanelRegion,
    },
    Float {
        target_position: (f32, f32),
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizeAxis {
    Width,
    Height,
}

// Animation easing functions
pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

pub fn ease_in_out_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

pub fn ease_out_elastic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    let p = 0.3;
    let s = p / 4.0;
    (2.0_f32).powf(-10.0 * t) * ((t - s) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
}

pub struct PanelAnimation {
    pub panel_id: super::node_types::PanelId,
    pub animation_type: AnimationType,
    pub progress: f32,
    pub duration_ms: u32,
    pub start_time: Instant,
    pub start_value: f32,
    pub target_value: f32,
    pub easing_fn: fn(f32) -> f32,
    pub completed: bool,
}

impl PanelAnimation {
    pub fn new_resize(
        panel_id: super::node_types::PanelId,
        from: f32,
        to: f32,
        duration_ms: u32,
    ) -> Self {
        Self {
            panel_id,
            animation_type: AnimationType::Resize {
                axis: ResizeAxis::Width,
            },
            progress: 0.0,
            duration_ms,
            start_time: Instant::now(),
            start_value: from,
            target_value: to,
            easing_fn: ease_out_cubic,
            completed: false,
        }
    }

    pub fn new_collapse(
        panel_id: super::node_types::PanelId,
        target_collapsed: bool,
        duration_ms: u32,
    ) -> Self {
        Self {
            panel_id,
            animation_type: AnimationType::Collapse { target_collapsed },
            progress: 0.0,
            duration_ms,
            start_time: Instant::now(),
            start_value: if target_collapsed { 1.0 } else { 0.0 },
            target_value: if target_collapsed { 0.0 } else { 1.0 },
            easing_fn: ease_out_cubic,
            completed: false,
        }
    }

    pub fn new_dock(
        panel_id: super::node_types::PanelId,
        target_region: super::node_types::PanelRegion,
        duration_ms: u32,
    ) -> Self {
        Self {
            panel_id,
            animation_type: AnimationType::Dock { target_region },
            progress: 0.0,
            duration_ms,
            start_time: Instant::now(),
            start_value: 0.0,
            target_value: 1.0,
            easing_fn: ease_in_out_quad,
            completed: false,
        }
    }

    pub fn new_float(
        panel_id: super::node_types::PanelId,
        target_position: (f32, f32),
        duration_ms: u32,
    ) -> Self {
        Self {
            panel_id,
            animation_type: AnimationType::Float { target_position },
            progress: 0.0,
            duration_ms,
            start_time: Instant::now(),
            start_value: 0.0,
            target_value: 1.0,
            easing_fn: ease_out_cubic,
            completed: false,
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        self.progress = (elapsed / self.duration_ms as f32).min(1.0);

        if self.progress >= 1.0 {
            self.completed = true;
        }
    }

    pub fn current_value(&self) -> f32 {
        let eased = (self.easing_fn)(self.progress);
        self.start_value + (self.target_value - self.start_value) * eased
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn cancel(&mut self) {
        self.completed = true;
    }
}

// Animation queue manager
pub struct AnimationManager {
    animations: Vec<PanelAnimation>,
    max_animations: usize,
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            max_animations: 10,
        }
    }

    pub fn push(&mut self, animation: PanelAnimation) {
        // Cancel existing animation for same panel
        self.animations.retain(|a| a.panel_id != animation.panel_id);

        // Add new animation
        self.animations.push(animation);

        // Limit queue size
        if self.animations.len() > self.max_animations {
            self.animations.remove(0);
        }
    }

    pub fn push_resize(
        &mut self,
        panel_id: super::node_types::PanelId,
        from: f32,
        to: f32,
        duration_ms: u32,
    ) {
        self.push(PanelAnimation::new_resize(panel_id, from, to, duration_ms));
    }

    pub fn push_collapse(
        &mut self,
        panel_id: super::node_types::PanelId,
        target_collapsed: bool,
        duration_ms: u32,
    ) {
        self.push(PanelAnimation::new_collapse(
            panel_id,
            target_collapsed,
            duration_ms,
        ));
    }

    pub fn push_dock(
        &mut self,
        panel_id: super::node_types::PanelId,
        target_region: super::node_types::PanelRegion,
        duration_ms: u32,
    ) {
        self.push(PanelAnimation::new_dock(
            panel_id,
            target_region,
            duration_ms,
        ));
    }

    pub fn push_float(
        &mut self,
        panel_id: super::node_types::PanelId,
        target_position: (f32, f32),
        duration_ms: u32,
    ) {
        self.push(PanelAnimation::new_float(
            panel_id,
            target_position,
            duration_ms,
        ));
    }

    pub fn update(&mut self) {
        for animation in &mut self.animations {
            animation.update();
        }

        // Remove completed animations
        self.animations.retain(|a| !a.is_completed());
    }

    pub fn get_animation(&self, panel_id: super::node_types::PanelId) -> Option<&PanelAnimation> {
        self.animations.iter().find(|a| a.panel_id == panel_id)
    }

    pub fn get_value(&self, panel_id: super::node_types::PanelId) -> Option<f32> {
        self.get_animation(panel_id).map(|a| a.current_value())
    }

    pub fn cancel(&mut self, panel_id: super::node_types::PanelId) {
        if let Some(a) = self.animations.iter_mut().find(|a| a.panel_id == panel_id) {
            a.cancel();
        }
    }

    pub fn cancel_all(&mut self) {
        for a in &mut self.animations {
            a.cancel();
        }
    }

    pub fn is_animating(&self, panel_id: super::node_types::PanelId) -> bool {
        self.animations
            .iter()
            .any(|a| a.panel_id == panel_id && !a.is_completed())
    }

    pub fn is_any_animating(&self) -> bool {
        self.animations.iter().any(|a| !a.is_completed())
    }

    pub fn count(&self) -> usize {
        self.animations.len()
    }

    pub fn clear_completed(&mut self) {
        self.animations.retain(|a| !a.is_completed());
    }
}
