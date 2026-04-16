// Layout Node Types - Coral Engine
// Core data structures for layout tree

use std::fmt;

pub type PanelId = &'static str;
pub type FloatingId = u64;
pub type ViewportId = u64;

// Panel regions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PanelRegion {
    Left,
    Right,
    Top,
    Bottom,
    Center,
}

impl PanelRegion {
    pub fn name(&self) -> &'static str {
        match self {
            PanelRegion::Left => "Left",
            PanelRegion::Right => "Right",
            PanelRegion::Top => "Top",
            PanelRegion::Bottom => "Bottom",
            PanelRegion::Center => "Center",
        }
    }
}

// Split direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

// Viewport type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewportType {
    Perspective3D,
    TopDown,
    Front,
    Side,
    UVEditor,
    GraphEditor,
    Camera,
}

// Panel content types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PanelContent {
    SceneOutliner,
    Properties,
    OceanConfig,
    Stats,
    Controls,
    Custom(PanelId),
}

impl PanelContent {
    pub fn panel_id(&self) -> PanelId {
        match self {
            PanelContent::SceneOutliner => "outliner",
            PanelContent::Properties => "properties",
            PanelContent::OceanConfig => "ocean_config",
            PanelContent::Stats => "stats",
            PanelContent::Controls => "controls",
            PanelContent::Custom(id) => id,
        }
    }

    pub fn title(&self) -> String {
        match self {
            PanelContent::SceneOutliner => "Outliner".to_string(),
            PanelContent::Properties => "Properties".to_string(),
            PanelContent::OceanConfig => "Ocean Config".to_string(),
            PanelContent::Stats => "Statistics".to_string(),
            PanelContent::Controls => "Controls".to_string(),
            PanelContent::Custom(id) => id.to_string(),
        }
    }
}

// Split node
#[derive(Clone, Debug)]
pub struct SplitNode {
    pub direction: SplitDirection,
    pub ratio: f32,
    pub min_ratio: f32,
    pub children: [Box<LayoutNode>; 2],
    pub grip_size: f32,
    pub collapsible: bool,
}

// Panel node
#[derive(Clone, Debug)]
pub struct PanelNode {
    pub id: PanelId,
    pub region: PanelRegion,
    pub title: String,
    pub min_size: f32,
    pub max_size: f32,
    pub collapsed: bool,
    pub content: PanelContent,
}

// Floating node
#[derive(Clone, Debug)]
pub struct FloatingNode {
    pub id: FloatingId,
    pub parent_panel: Option<PanelId>,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub title: String,
    pub dockable: bool,
    pub minimizeable: bool,
    pub maximizeable: bool,
    pub content: PanelContent,
}

// Viewport node
#[derive(Clone, Debug)]
pub struct ViewportNode {
    pub id: ViewportId,
    pub viewport_type: ViewportType,
    pub show_grid: bool,
    pub show_axes: bool,
    pub background_color: [f32; 3],
}

// Layout node enum
#[derive(Clone)]
pub enum LayoutNode {
    Panel(PanelNode),
    Split(SplitNode),
    Floating(FloatingNode),
    Viewport(ViewportNode),
}

impl fmt::Debug for LayoutNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LayoutNode::Panel(p) => write!(f, "Panel({})", p.id),
            LayoutNode::Split(s) => write!(f, "Split({:?})", s.direction),
            LayoutNode::Floating(fl) => write!(f, "Floating({})", fl.title),
            LayoutNode::Viewport(v) => write!(f, "Viewport({:?})", v.viewport_type),
        }
    }
}
