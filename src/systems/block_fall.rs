use amethyst::{
    assets::Handle,
    core::Time,
    derive::SystemDesc,
    ecs::{
        prelude::{
            Join, Read, ReadExpect, ReadStorage, ReaderId, System, SystemData, Write, WriteStorage,
        },
        Entities,
    },
    renderer::{SpriteRender, SpriteSheet},
    shrev::EventChannel,
};

use crate::{
    entities::{Block, DeadBlock, Position},
    events::{BlockLandEvent, ResetFallTimerEvent},
};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::renderer::resources::Tint;

const FALL_TIMER: f32 = 0.4;

#[derive(SystemDesc)]
pub struct BlockFallSystem {
    fall_timer: f32, // Seconds until next step down
    reader_id: Option<ReaderId<ResetFallTimerEvent>>,
}

impl BlockFallSystem {
    pub fn new() -> Self {
        Self {
            fall_timer: FALL_TIMER,
            reader_id: None,
        }
    }
}

impl<'s> System<'s> for BlockFallSystem {
    type SystemData = (
        ReadStorage<'s, Block>,
        WriteStorage<'s, DeadBlock>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Entities<'s>,
        Write<'s, EventChannel<BlockLandEvent>>,
        Write<'s, EventChannel<ResetFallTimerEvent>>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, Tint>,
    );

    fn run(
        &mut self,
        (
            blocks,
            mut dead_blocks,
            mut positions,
            mut transforms,
            time,
            entities,
            mut land_channel,
            mut reset_channel,
            mut sprite_renders,
            sprite_sheet_handle,
            mut tints,
        ): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| reset_channel.register_reader());

        for _ in reset_channel.read(reader_id) {
            self.fall_timer = FALL_TIMER;
        }

        self.fall_timer -= time.delta_seconds();

        if self.fall_timer <= 0.0 {
            self.fall_timer = FALL_TIMER; // TODO: Make this change over time and centralize it to a variable

            let dead_positions = (&mut dead_blocks, &mut positions)
                .join()
                .map(|(_, pos)| *pos)
                .collect::<Vec<_>>();

            let mut new_dead_blocks = Vec::<(DeadBlock, Position)>::new();
            for (entity, block, position) in (&*entities, &blocks, &mut positions).join() {
                let mut collide = false;

                'self_loop: for self_pos in block.get_filled_positions(position) {
                    if self_pos.row == 0 {
                        collide = true;
                        break;
                    }

                    for other_pos in &dead_positions {
                        let pos_below_self = Position {
                            row: self_pos.row - 1,
                            col: self_pos.col,
                        };
                        if pos_below_self == *other_pos {
                            collide = true;
                            break 'self_loop;
                        }
                    }
                }

                if collide {
                    for new_dead_pos in block.get_filled_positions(position) {
                        new_dead_blocks.push((DeadBlock::new(block.block_type), new_dead_pos));
                    }
                    entities.delete(entity).unwrap();

                    land_channel.single_write(BlockLandEvent {});
                } else {
                    position.row -= 1;
                }
            }

            for (new_dead_block, new_pos) in new_dead_blocks {
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet_handle.clone(),
                    sprite_number: 0,
                };

                let mut sprite_transform = Transform::default();
                sprite_transform.set_scale(Vector3::new(0.065, 0.065, 1.0));
                sprite_transform.set_translation_xyz(
                    new_pos.col as f32 + 0.5,
                    new_pos.row as f32 + 0.5,
                    0.0,
                );

                let tint = Tint(new_dead_block.block_type.get_color());

                entities
                    .build_entity()
                    .with(new_dead_block, &mut dead_blocks)
                    .with(new_pos, &mut positions)
                    .with(sprite_render, &mut sprite_renders)
                    .with(sprite_transform, &mut transforms)
                    .with(tint, &mut tints)
                    .build();
            }
        }
    }
}
