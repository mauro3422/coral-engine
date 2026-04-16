// Layout State - Coral Engine
// Manages layout tree and default Blender-style layout

use super::node_types::{
    FloatingId, FloatingNode, LayoutNode, PanelContent, PanelId, PanelNode, PanelRegion,
    SplitDirection, SplitNode, ViewportId, ViewportNode, ViewportType,
};
use std::collections::HashMap;

pub struct LayoutState {
    pub root: Box<LayoutNode>,
    pub floating: Vec<FloatingNode>,
    pub active_panel: Option<PanelId>,
    pub active_viewport: ViewportId,
    pub layouts: HashMap<String, Box<LayoutNode>>,
    pub default_layout: String,
    next_floating_id: FloatingId,
    next_viewport_id: ViewportId,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self::blender_default()
    }
}

impl LayoutState {
    pub fn new() -> Self {
        Self::blender_default()
    }

    pub fn blender_default() -> Self {
        let outliner = PanelNode {
            id: "outliner",
            region: PanelRegion::Left,
            title: "Outliner".to_string(),
            min_size: 150.0,
            max_size: 400.0,
            collapsed: false,
            content: PanelContent::SceneOutliner,
        };

        let properties = PanelNode {
            id: "properties",
            region: PanelRegion::Right,
            title: "Properties".to_string(),
            min_size: 200.0,
            max_size: 500.0,
            collapsed: false,
            content: PanelContent::Properties,
        };

        let stats = PanelNode {
            id: "stats",
            region: PanelRegion::Right,
            title: "Statistics".to_string(),
            min_size: 100.0,
            max_size: 300.0,
            collapsed: false,
            content: PanelContent::Stats,
        };

        let right_split = SplitNode {
            direction: SplitDirection::Vertical,
            ratio: 0.7,
            min_ratio: 0.3,
            children: [
                Box::new(LayoutNode::Panel(properties)),
                Box::new(LayoutNode::Panel(stats)),
            ],
            grip_size: 4.0,
            collapsible: true,
        };

        let main_split = SplitNode {
            direction: SplitDirection::Horizontal,
            ratio: 0.2,
            min_ratio: 0.1,
            children: [
                Box::new(LayoutNode::Panel(outliner)),
                Box::new(LayoutNode::Split(right_split)),
            ],
            grip_size: 4.0,
            collapsible: false,
        };

        let viewport = ViewportNode {
            id: 1,
            viewport_type: ViewportType::Perspective3D,
            show_grid: true,
            show_axes: true,
            background_color: [0.4, 0.6, 0.9],
        };

        let layout_with_viewport = SplitNode {
            direction: SplitDirection::Horizontal,
            ratio: 1.0,
            min_ratio: 0.0,
            children: [
                Box::new(LayoutNode::Split(main_split)),
                Box::new(LayoutNode::Viewport(viewport.clone())),
            ],
            grip_size: 0.0,
            collapsible: false,
        };

        let root = Box::new(LayoutNode::Split(layout_with_viewport.clone()));

        let mut layouts = HashMap::new();
        layouts.insert("default".to_string(), root.clone());

        Self {
            root,
            floating: Vec::new(),
            active_panel: None,
            active_viewport: 1,
            layouts,
            default_layout: "default".to_string(),
            next_floating_id: 1,
            next_viewport_id: 2,
        }
    }

    // === Layout Management ===

    pub fn load_layout(&mut self, name: &str) -> bool {
        if let Some(root) = self.layouts.remove(name) {
            self.root = root;
            true
        } else {
            false
        }
    }

    pub fn save_layout(&mut self, name: String) {
        self.layouts.insert(name, self.root.clone());
    }

    pub fn reset_to_default(&mut self) {
        if let Some(default_root) = self.layouts.get(&self.default_layout).cloned() {
            self.root = default_root;
        }
    }

    // === Panel Operations ===

    pub fn add_panel(
        &mut self,
        region: PanelRegion,
        content: PanelContent,
        title: &str,
    ) -> PanelId {
        let id = content.panel_id();

        let panel = PanelNode {
            id,
            region,
            title: title.to_string(),
            min_size: 150.0,
            max_size: 500.0,
            collapsed: false,
            content,
        };

        self.root = Box::new(LayoutNode::Panel(panel));
        id
    }

    pub fn collapse_panel(&mut self, _id: PanelId) {
        // TODO: recursively find and toggle collapse
    }

    // === Floating Operations ===

    pub fn create_floating(
        &mut self,
        content: PanelContent,
        title: &str,
        position: (f32, f32),
        size: (f32, f32),
    ) -> FloatingId {
        let id = self.next_floating_id;
        self.next_floating_id += 1;

        let floating = FloatingNode {
            id,
            parent_panel: None,
            position,
            size,
            title: title.to_string(),
            dockable: true,
            minimizeable: true,
            maximizeable: true,
            content,
        };

        self.floating.push(floating);
        id
    }

    pub fn dock_floating(&mut self, _floating_id: FloatingId, _region: PanelRegion) {
        // TODO: convert floating to docked panel
    }

    pub fn close_floating(&mut self, floating_id: FloatingId) {
        self.floating.retain(|f| f.id != floating_id);
    }

    pub fn move_floating(&mut self, floating_id: FloatingId, position: (f32, f32)) {
        if let Some(f) = self.floating.iter_mut().find(|f| f.id == floating_id) {
            f.position = position;
        }
    }

    pub fn resize_floating(&mut self, floating_id: FloatingId, size: (f32, f32)) {
        if let Some(f) = self.floating.iter_mut().find(|f| f.id == floating_id) {
            f.size = size;
        }
    }

    // === Viewport Operations ===

    pub fn create_viewport(&mut self, viewport_type: ViewportType) -> ViewportId {
        let id = self.next_viewport_id;
        self.next_viewport_id += 1;

        let viewport = ViewportNode {
            id,
            viewport_type,
            show_grid: true,
            show_axes: true,
            background_color: [0.4, 0.6, 0.9],
        };

        // Add to layout tree (simplified - adds as new branch)
        self.root = Box::new(LayoutNode::Viewport(viewport));

        id
    }

    pub fn set_active_viewport(&mut self, id: ViewportId) {
        self.active_viewport = id;
    }

    pub fn close_viewport(&mut self, id: ViewportId) {
        if id != 1 {
            // Don't close main viewport
            self.root = Box::new(LayoutNode::Viewport(ViewportNode {
                id: 1,
                viewport_type: ViewportType::Perspective3D,
                show_grid: true,
                show_axes: true,
                background_color: [0.4, 0.6, 0.9],
            }));
            if self.active_viewport == id {
                self.active_viewport = 1;
            }
        }
    }

    // === Utility ===

    pub fn is_panel_visible(&self, id: PanelId) -> bool {
        self.find_panel(&self.root, id)
            .map(|p| !p.collapsed)
            .unwrap_or(true)
    }

    fn find_panel<'a>(&self, node: &'a LayoutNode, id: PanelId) -> Option<&'a PanelNode> {
        match node {
            LayoutNode::Panel(p) if p.id == id => Some(p),
            LayoutNode::Split(s) => self
                .find_panel(&s.children[0], id)
                .or_else(|| self.find_panel(&s.children[1], id)),
            _ => None,
        }
    }

    pub fn panel_count(&self) -> usize {
        self.count_panels(&self.root)
    }

    fn count_panels(&self, node: &LayoutNode) -> usize {
        match node {
            LayoutNode::Panel(_) => 1,
            LayoutNode::Split(s) => {
                self.count_panels(&s.children[0]) + self.count_panels(&s.children[1])
            }
            LayoutNode::Floating(_) => 0,
            LayoutNode::Viewport(_) => 0,
        }
    }
}
