// Splitter System - Coral Engine
// Handles drag detection and resize calculations

use super::node_types::SplitDirection;

#[derive(Clone, Debug)]
pub struct SplitterState {
    pub split_id: usize,
    pub is_dragging: bool,
    pub drag_start: Option<(f32, f32)>,
    pub start_ratio: f32,
    pub current_ratio: f32,
}

impl SplitterState {
    pub fn new(id: usize) -> Self {
        Self {
            split_id: id,
            is_dragging: false,
            drag_start: None,
            start_ratio: 0.5,
            current_ratio: 0.5,
        }
    }

    pub fn hit_test(&self, pos: (f32, f32), rect: (f32, f32, f32, f32)) -> bool {
        // rect: (min_x, min_y, max_x, max_y)
        pos.0 >= rect.0 && pos.0 <= rect.2 && pos.1 >= rect.1 && pos.1 <= rect.3
    }

    pub fn start_drag(&mut self, pos: (f32, f32), current_ratio: f32) {
        self.is_dragging = true;
        self.drag_start = Some(pos);
        self.start_ratio = current_ratio;
        self.current_ratio = current_ratio;
    }

    pub fn update_drag(
        &mut self,
        pos: (f32, f32),
        total_size: f32,
        min_ratio: f32,
        max_ratio: f32,
    ) {
        if self.is_dragging {
            if let Some(start) = self.drag_start {
                let delta = match total_size {
                    s if s > 0.0 => (pos.0 - start.0) / s,
                    _ => 0.0,
                };
                self.current_ratio = (self.start_ratio + delta).clamp(min_ratio, max_ratio);
            }
        }
    }

    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.drag_start = None;
    }

    pub fn reset(&mut self) {
        self.is_dragging = false;
        self.drag_start = None;
        self.current_ratio = 0.5;
    }
}

// Splitter manager for multiple splitters
pub struct SplitterManager {
    splitters: Vec<SplitterState>,
    active_splitter: Option<usize>,
}

impl Default for SplitterManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitterManager {
    pub fn new() -> Self {
        Self {
            splitters: Vec::new(),
            active_splitter: None,
        }
    }

    pub fn register_splitter(&mut self, id: usize) {
        if self.splitters.iter().all(|s| s.split_id != id) {
            self.splitters.push(SplitterState::new(id));
        }
    }

    pub fn unregister_splitter(&mut self, id: usize) {
        self.splitters.retain(|s| s.split_id != id);
        if self.active_splitter == Some(id) {
            self.active_splitter = None;
        }
    }

    pub fn try_start_drag(&mut self, id: usize, pos: (f32, f32), current_ratio: f32) -> bool {
        if let Some(splitter) = self.splitters.iter_mut().find(|s| s.split_id == id) {
            splitter.start_drag(pos, current_ratio);
            self.active_splitter = Some(id);
            true
        } else {
            false
        }
    }

    pub fn update_drag(
        &mut self,
        pos: (f32, f32),
        total_size: f32,
        min_ratio: f32,
        max_ratio: f32,
    ) {
        if let Some(active) = self.active_splitter {
            if let Some(splitter) = self.splitters.iter_mut().find(|s| s.split_id == active) {
                splitter.update_drag(pos, total_size, min_ratio, max_ratio);
            }
        }
    }

    pub fn end_drag(&mut self) {
        if let Some(active) = self.active_splitter {
            if let Some(splitter) = self.splitters.iter_mut().find(|s| s.split_id == active) {
                splitter.end_drag();
            }
            self.active_splitter = None;
        }
    }

    pub fn get_ratio(&self, id: usize) -> Option<f32> {
        self.splitters
            .iter()
            .find(|s| s.split_id == id)
            .map(|s| s.current_ratio)
    }

    pub fn is_dragging(&self, id: usize) -> bool {
        self.splitters
            .iter()
            .any(|s| s.split_id == id && s.is_dragging)
    }

    pub fn is_any_dragging(&self) -> bool {
        self.splitters.iter().any(|s| s.is_dragging)
    }

    pub fn clear_all(&mut self) {
        for splitter in &mut self.splitters {
            splitter.reset();
        }
        self.active_splitter = None;
    }
}

// Size calculation helpers
pub fn calculate_split_sizes(
    total_size: f32,
    ratio: f32,
    min_ratio: f32,
    max_ratio: f32,
) -> (f32, f32) {
    let ratio = ratio.clamp(min_ratio, max_ratio);
    let first = total_size * ratio;
    let second = total_size - first;
    (first, second)
}

pub fn calculate_grip_rect(
    split_dir: SplitDirection,
    first_size: f32,
    total_pos: (f32, f32),
    grip_size: f32,
) -> (f32, f32, f32, f32) {
    match split_dir {
        SplitDirection::Horizontal => {
            // Vertical grip between left/right
            (
                total_pos.0 + first_size - grip_size / 2.0,
                total_pos.1,
                total_pos.0 + first_size + grip_size / 2.0,
                total_pos.1 + 1000.0, // Full height
            )
        }
        SplitDirection::Vertical => {
            // Horizontal grip between top/bottom
            (
                total_pos.0,
                total_pos.1 + first_size - grip_size / 2.0,
                total_pos.0 + 1000.0, // Full width
                total_pos.1 + first_size + grip_size / 2.0,
            )
        }
    }
}
