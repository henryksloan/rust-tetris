use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::entities::{Block, DeadBlock, Position, BOARD_WIDTH};

#[derive(SystemDesc)]
pub struct BlockInputSystem {
    last_movement: f32,
    rotated_last: bool,
}

impl BlockInputSystem {
    pub fn new() -> Self {
        Self {
            last_movement: 0.0,
            rotated_last: false,
        }
    }
}

impl<'s> System<'s> for BlockInputSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        WriteStorage<'s, DeadBlock>,
        WriteStorage<'s, Position>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut blocks, mut dead_blocks, mut positions, input): Self::SystemData) {
        let dead_positions = (&mut dead_blocks, &mut positions)
            .join()
            .map(|(_, pos)| pos.clone())
            .collect::<Vec<_>>();

        'block_loop: for (block, position) in (&mut blocks, &mut positions).join() {
            let mut movement = input.axis_value("horizontal").unwrap_or(0.0);
            let mut same_movement = false;
            if (self.last_movement > 0.0 && movement > 0.0)
                || (self.last_movement < 0.0 && movement < 0.0)
            {
                same_movement = true;
            }
            self.last_movement = movement;
            if same_movement {
                movement = 0.0;
            }

            let mut new_position = position.clone();
            new_position.col += if movement > 0.0 { -1 } else { 1 };

            let mut new_block = Block {
                block_type: block.block_type,
                rotation: block.rotation,
            };

            let mut rotated = input.action_is_down("rotate").unwrap_or(false);
            if self.rotated_last && !rotated {
                self.rotated_last = false;
            } else if !self.rotated_last && rotated {
                self.rotated_last = true;
            } else if self.rotated_last && rotated {
                rotated = false;
            }

            if rotated {
                println!("Rotate");
                new_block.rotate_left();
            } else if movement == 0.0 {
                continue;
            }

            for self_pos in block.get_filled_positions(&new_position) {
                if self_pos.col < 0 || self_pos.col >= BOARD_WIDTH as i8 {
                    continue 'block_loop;
                }

                for other_pos in &dead_positions {
                    if self_pos == *other_pos {
                        println!("Moved into dead");
                        continue 'block_loop;
                    }
                }
            }

            position.col = new_position.col;
            block.rotation = new_block.rotation;
        }
    }
}
