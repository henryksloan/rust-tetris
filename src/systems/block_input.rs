use amethyst::{
    core::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::EventChannel,
};

use crate::{
    entities::{Block, DeadBlock, Position, BOARD_WIDTH},
    events::ResetFallTimerEvent,
};
use std::collections::{HashMap, HashSet};

#[derive(SystemDesc)]
pub struct BlockInputSystem {
    last_actions: HashSet<String>,
    action_timers: HashMap<String, f32>,
}

impl BlockInputSystem {
    pub fn new() -> Self {
        Self {
            last_actions: HashSet::new(),
            action_timers: HashMap::new(),
        }
    }

    fn action_no_spam(&mut self, input: &InputHandler<StringBindings>, name: &str) -> bool {
        let contains = self.last_actions.contains(name);
        let action = input.action_is_down(name).unwrap_or(false);
        if contains && !action {
            self.last_actions.remove(name);
        } else if !contains && action {
            self.last_actions.insert(String::from(name));
        } else if contains && action {
            return false;
        }

        action
    }

    fn action_with_timer<T: PartialEq>(
        &mut self,
        time: &Time,
        default_seconds: f32,
        name: &str,
        value: T,
        default_value: T,
    ) -> T {
        let timer = self
            .action_timers
            .entry(String::from(name))
            .or_insert(default_seconds);

        if *timer <= 0.0 {
            if value != default_value {
                *timer = default_seconds;
            }
            value
        } else if value == default_value {
            *timer = 0.0;
            default_value
        } else {
            *timer -= time.delta_seconds();
            default_value
        }
    }

    fn position_collides(block: &Block, position: &Position, dead_positions: &[Position]) -> bool {
        for self_pos in block.get_filled_positions(&position) {
            let outside_bounds =
                || self_pos.col < 0 || self_pos.col >= BOARD_WIDTH as i8 || self_pos.row < 0;
            let in_dead = || dead_positions.iter().any(|dead_pos| self_pos == *dead_pos);
            if outside_bounds() || in_dead() {
                return true;
            }
        }

        false
    }

    fn hard_drop(block: &Block, position: &mut Position, dead_positions: &[Position]) {
        let down_collides = |pos: &Position| {
            let down_pos = Position {
                row: pos.row - 1,
                col: pos.col,
            };

            Self::position_collides(block, &down_pos, dead_positions)
        };

        while !down_collides(position) {
            position.row -= 1;
        }
    }
}

impl<'s> System<'s> for BlockInputSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        WriteStorage<'s, DeadBlock>,
        WriteStorage<'s, Position>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, EventChannel<ResetFallTimerEvent>>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut blocks, mut dead_blocks, mut positions, input, mut reset_channel, time): Self::SystemData,
    ) {
        let dead_positions = (&mut dead_blocks, &mut positions)
            .join()
            .map(|(_, pos)| *pos)
            .collect::<Vec<_>>();

        'block_loop: for (block, position) in (&mut blocks, &mut positions).join() {
            if self.action_no_spam(&*input, &"hard_drop".to_string()) {
                Self::hard_drop(block, position, &dead_positions);
            }

            let movement_input = input.axis_value("horizontal").unwrap_or(0.0);
            let movement = self.action_with_timer(&*time, 0.14, "horizontal", movement_input, 0.0);

            let soft_drop_input = input.action_is_down("soft_drop").unwrap_or(false);
            let soft_drop =
                self.action_with_timer(&*time, 0.14, "soft_drop", soft_drop_input, false);

            let new_position = Position {
                row: position.row - soft_drop as i8,
                col: position.col - movement as i8,
            };

            let mut new_block = Block {
                block_type: block.block_type,
                rotation: block.rotation,
            };

            let rotated = self.action_no_spam(&*input, &"rotate".to_string());
            if rotated {
                new_block.rotate_left();
            } else if movement == 0.0 && !soft_drop {
                continue;
            }

            if Self::position_collides(&new_block, &new_position, &dead_positions) {
                continue 'block_loop;
            }

            if position.row != new_position.row {
                reset_channel.single_write(ResetFallTimerEvent {});
            }

            position.row = new_position.row;
            position.col = new_position.col;
            block.rotation = new_block.rotation;
        }
    }
}
