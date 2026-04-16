// WaterBlock Registry - Manages all water blocks with metadata
// Provides registration, lookup, and organization by tags

use crate::ocean::block::{BlockPos, WaterBlock};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockTag(pub String);

impl BlockTag {
    pub fn new(name: &str) -> Self {
        Self(name.to_lowercase())
    }
}

#[derive(Clone, Debug)]
pub struct BlockMetadata {
    pub name: String,
    pub tags: Vec<BlockTag>,
    pub position: BlockPos,
    pub hidden: bool,
    pub locked: bool,
}

impl BlockMetadata {
    pub fn new(name: String, position: BlockPos) -> Self {
        Self {
            name,
            tags: Vec::new(),
            position,
            hidden: false,
            locked: false,
        }
    }

    pub fn with_tag(mut self, tag: BlockTag) -> Self {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self
    }

    pub fn has_tag(&self, tag: &BlockTag) -> bool {
        self.tags.contains(tag)
    }
}

pub struct BlockRegistry {
    pub blocks: HashMap<BlockPos, WaterBlock>,
    pub metadata: HashMap<BlockPos, BlockMetadata>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn register(&mut self, block: WaterBlock, name: Option<String>) -> BlockPos {
        let pos = block.position;
        let default_name = name.unwrap_or_else(|| format!("Block_{}_{}", pos.x, pos.z));

        let meta = BlockMetadata::new(default_name, pos);
        self.metadata.insert(pos, meta);
        self.blocks.insert(pos, block);

        pos
    }

    pub fn get(&self, pos: BlockPos) -> Option<&WaterBlock> {
        self.blocks.get(&pos)
    }

    pub fn get_mut(&mut self, pos: BlockPos) -> Option<&mut WaterBlock> {
        self.blocks.get_mut(&pos)
    }

    pub fn get_metadata(&self, pos: BlockPos) -> Option<&BlockMetadata> {
        self.metadata.get(&pos)
    }

    pub fn get_metadata_mut(&mut self, pos: BlockPos) -> Option<&mut BlockMetadata> {
        self.metadata.get_mut(&pos)
    }

    pub fn unregister(&mut self, pos: BlockPos) -> bool {
        if self.blocks.remove(&pos).is_some() {
            self.metadata.remove(&pos);
            true
        } else {
            false
        }
    }

    pub fn find_by_tag(&self, tag: &BlockTag) -> Vec<BlockPos> {
        self.metadata
            .iter()
            .filter(|(_, m)| m.has_tag(tag))
            .map(|(pos, _)| *pos)
            .collect()
    }

    pub fn find_by_name(&self, name: &str) -> Option<BlockPos> {
        self.metadata
            .iter()
            .find(|(_, m)| m.name.to_lowercase() == name.to_lowercase())
            .map(|(pos, _)| *pos)
    }

    pub fn all_positions(&self) -> Vec<BlockPos> {
        self.blocks.keys().copied().collect()
    }

    pub fn count(&self) -> usize {
        self.blocks.len()
    }
}

impl Default for BlockRegistry {
    fn default() -> Self {
        Self::new()
    }
}
