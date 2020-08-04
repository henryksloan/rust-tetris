use crate::entities::block::BlockType;

use amethyst::ecs::{Component, DenseVecStorage};

pub struct DeadBlock {
    pub block_type: BlockType,
}

impl Component for DeadBlock {
    type Storage = DenseVecStorage<Self>;
}

impl DeadBlock {
    pub fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }
}
