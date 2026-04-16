// Layout System - Coral Engine v0.5.0

pub mod animation;
pub mod layout_state;
pub mod node_types;
pub mod panel_registry;
pub mod splitter;
pub mod viewport_manager;

#[allow(unused_imports)]
pub use node_types::PanelId;
#[allow(unused_imports)]
pub use node_types::PanelRegion;

#[allow(unused_imports)]
pub use animation::AnimationManager;
#[allow(unused_imports)]
pub use animation::PanelAnimation;
pub use layout_state::LayoutState;

#[allow(unused_imports)]
pub use panel_registry::{PanelContentTrait, PanelRegistry};

#[allow(unused_imports)]
pub use splitter::{SplitterManager, SplitterState};

#[allow(unused_imports)]
pub use viewport_manager::{ViewportManager, ViewportState};
