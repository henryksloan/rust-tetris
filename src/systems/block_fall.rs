use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::{
        prelude::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
        Entities,
    },
    shrev::EventChannel,
};

use crate::{
    entities::{Block, DeadBlock, Position},
    events::BlockLandEvent,
};

const FALL_TIMER: f32 = 0.4;

#[derive(SystemDesc)]
pub struct BlockFallSystem {
    fall_timer: f32, // Seconds until next step down
}

impl BlockFallSystem {
    pub fn new() -> Self {
        Self {
            fall_timer: FALL_TIMER,
        }
    }
}

impl<'s> System<'s> for BlockFallSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        WriteStorage<'s, DeadBlock>,
        WriteStorage<'s, Position>,
        Read<'s, Time>,
        Entities<'s>,
        Write<'s, EventChannel<BlockLandEvent>>,
    );

    fn run(
        &mut self,
        (blocks, mut dead_blocks, mut positions, time, entities, mut land_channel): Self::SystemData,
    ) {
        self.fall_timer -= time.delta_seconds();

        if self.fall_timer <= 0.0 {
            self.fall_timer = FALL_TIMER; // TODO: Make this change over time and centralize it to a variable

            let dead_positions = (&mut dead_blocks, &mut positions)
                .join()
                .map(|(_, pos)| pos.clone())
                .collect::<Vec<_>>();

            let mut new_dead_blocks = Vec::<(DeadBlock, Position)>::new();
            for (entity, block, position) in (&*entities, &blocks, &mut positions).join() {
                // TODO: First, check if moving it down would make it hit an immovable
                // If so, don't move and turn the block into several `DeadBlock`s
                let mut collide = false;

                'self_loop: for self_pos in block.get_filled_positions(position) {
                    // println!("Hey col:row {}:{}", self_pos.col, self_pos.row);
                    if self_pos.row == 0 {
                        println!("Collided with row==0");
                        collide = true;
                        break;
                    }

                    for other_pos in &dead_positions {
                        let mut pos_below_self = self_pos;
                        pos_below_self.row -= 1;
                        if pos_below_self == *other_pos {
                            println!("Collided with dead");
                            collide = true;
                            break 'self_loop;
                        }
                    }
                }

                // TODO: At 12:38 AM, 7/21/20, I figured out the crash
                // The implementation of blocks is always 4x4, but it doesn't really start at the bottom of the 4x4
                // So it tries to go under the screen before the actual squares in the tetromino reach row 0

                if collide {
                    for new_dead_pos in block.get_filled_positions(position) {
                        new_dead_blocks
                            .push((DeadBlock::new(block.block_type), new_dead_pos.clone()));
                    }
                    entities.delete(entity).unwrap();

                    land_channel.single_write(BlockLandEvent {});
                } else {
                    position.row -= 1;
                }
            }

            for (new_dead_block, new_pos) in new_dead_blocks {
                entities
                    .build_entity()
                    .with(new_dead_block, &mut dead_blocks)
                    .with(new_pos, &mut positions)
                    .build();
            }
        }
    }
}
