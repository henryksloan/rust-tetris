use amethyst::{
    ecs::{Component, DenseVecStorage},
    renderer::palette::rgb::Srgba,
};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::entities::position::Position;

pub struct Block {
    pub block_type: BlockType,
    pub rotation: u8,
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            rotation: 0,
        }
    }

    pub fn rotate_left(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn get_filled_positions(&self, pos: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        let shape: BlockShape = self.block_type.get_shape(self.rotation);
        for row in 0..4 {
            for col in 0..4 {
                if (shape & (1 << (row * 4 + col))) != 0 {
                    positions.push(Position {
                        row: pos.row + (3 - row),
                        col: pos.col + col,
                    });
                }
            }
        }
        positions
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

// https://tetris.fandom.com/wiki/SRS
#[derive(Copy, Clone)]
pub enum BlockType {
    O,
    J,
    L,
    I,
    S,
    Z,
    T,
}

type BlockShape = u16;

impl BlockType {
    pub fn get_shape(&self, rotation: u8) -> BlockShape {
        // TODO: Move to config file (RON)
        let shapes = match *self {
            BlockType::O => [0xCC00, 0xCC00, 0xCC00, 0xCC00],
            BlockType::J => [0x44C0, 0x8E00, 0x6440, 0x0E20],
            BlockType::L => [0x4460, 0x0E80, 0xC440, 0x2E00],
            BlockType::I => [0x0F00, 0x2222, 0x00F0, 0x4444],
            BlockType::S => [0x06C0, 0x8C40, 0x6C00, 0x4620],
            BlockType::Z => [0x0C60, 0x4C80, 0xC600, 0x2640],
            BlockType::T => [0x0E40, 0x4C40, 0x4E00, 0x4640],
        };
        shapes[rotation as usize % 4]
    }

    pub fn get_color(&self) -> Srgba {
        match *self {
            BlockType::O => Srgba::new(0.94, 0.94, 0.0, 1.0),
            BlockType::J => Srgba::new(0.94, 0.63, 0.0, 1.0),
            BlockType::L => Srgba::new(0.0, 0.0, 0.94, 1.0),
            BlockType::I => Srgba::new(0.0, 0.94, 0.94, 1.0),
            BlockType::S => Srgba::new(0.0, 0.94, 0.0, 1.0),
            BlockType::Z => Srgba::new(0.94, 0.0, 0.0, 1.0),
            BlockType::T => Srgba::new(0.64, 0.0, 0.94, 1.0),
        }
    }
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.gen_range(0, 7) {
            0 => BlockType::O,
            1 => BlockType::J,
            2 => BlockType::L,
            3 => BlockType::I,
            4 => BlockType::S,
            5 => BlockType::Z,
            _ => BlockType::T,
        }
    }
}
