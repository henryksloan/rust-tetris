use amethyst::ecs::{Component, DenseVecStorage};

pub const BOARD_WIDTH: u32 = 10;
pub const BOARD_HEIGHT: u32 = 16;

pub struct Board {
    pub width: u32,
    pub height: u32,
}

impl Component for Board {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Board {
    fn default() -> Self {
        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        }
    }
}
