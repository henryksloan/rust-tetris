use crate::entities::{Block, DeadBlock, Position};

use amethyst::{
    core::math::Point2,
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
    renderer::debug_drawing::DebugLinesComponent,
};

#[derive(SystemDesc)]
pub struct RenderSystem;

impl<'s> System<'s> for RenderSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        ReadStorage<'s, DeadBlock>,
        ReadStorage<'s, Position>,
        WriteStorage<'s, DebugLinesComponent>,
    );

    fn run(&mut self, (blocks, dead_blocks, positions, mut debug_lines): Self::SystemData) {
        for debug_line in (&mut debug_lines).join() {
            debug_line.clear();
        }

        for (block, position) in (&blocks, &positions).join() {
            // println!("col:row {}:{}", position.col as f32, position.row as f32);

            for debug_line in (&mut debug_lines).join() {
                for self_pos in block.get_filled_positions(position) {
                    debug_line.add_rectangle_2d(
                        Point2::new(self_pos.col as f32, self_pos.row as f32),
                        Point2::new((self_pos.col + 1) as f32, (self_pos.row + 1) as f32),
                        0.0,
                        block.block_type.get_color(),
                    );
                }
            }
        }

        for (block, position) in (&dead_blocks, &positions).join() {
            // println!("C");
            for debug_line in (&mut debug_lines).join() {
                debug_line.add_rectangle_2d(
                    Point2::new(position.col as f32, position.row as f32),
                    Point2::new((position.col + 1) as f32, (position.row + 1) as f32),
                    0.0,
                    block.block_type.get_color(),
                );
            }
        }
    }
}
