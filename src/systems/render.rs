use crate::entities::{Block, DeadBlock, Position};

use amethyst::renderer::palette::Srgba;
use amethyst::{
    core::math::{Point2, Point3},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, WriteStorage},
    renderer::debug_drawing::DebugLinesComponent,
};

#[derive(SystemDesc)]
pub struct RenderSystem;

impl RenderSystem {
    fn draw_crossed_square(
        &self,
        debug_line: &mut DebugLinesComponent,
        position: &Position,
        color: Srgba,
    ) {
        debug_line.add_rectangle_2d(
            Point2::new(position.col as f32, position.row as f32),
            Point2::new((position.col + 1) as f32, (position.row + 1) as f32),
            0.0,
            color,
        );
        debug_line.add_line(
            Point3::new(position.col as f32, position.row as f32, 0.0),
            Point3::new((position.col + 1) as f32, (position.row + 1) as f32, 0.0),
            color,
        );
        debug_line.add_line(
            Point3::new(position.col as f32, (position.row + 1) as f32, 0.0),
            Point3::new((position.col + 1) as f32, position.row as f32, 0.0),
            color,
        );
    }
}

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
            for debug_line in (&mut debug_lines).join() {
                for self_pos in block.get_filled_positions(position) {
                    self.draw_crossed_square(debug_line, &self_pos, block.block_type.get_color());
                }
            }
        }

        for (block, position) in (&dead_blocks, &positions).join() {
            for debug_line in (&mut debug_lines).join() {
                self.draw_crossed_square(debug_line, position, block.block_type.get_color());
            }
        }
    }
}
