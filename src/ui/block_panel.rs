// Block Panel - Block browser UI state

use crate::ocean::BlockPos;

#[derive(Clone, Debug, Default)]
pub struct BlockBrowserState {
    pub filter_tag: String,
    pub filter_name: String,
    pub show_hidden: bool,
    pub selected_block: Option<BlockPos>,
}

impl BlockBrowserState {
    pub fn new() -> Self {
        Self {
            filter_tag: String::new(),
            filter_name: String::new(),
            show_hidden: false,
            selected_block: None,
        }
    }
}
