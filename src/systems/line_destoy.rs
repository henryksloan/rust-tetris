use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{Join, ReaderId, System, SystemData, Write, WriteStorage},
        Entities,
    },
    shrev::EventChannel,
};

use crate::{
    entities::{DeadBlock, Position, BOARD_WIDTH},
    events::BlockLandEvent,
};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(SystemDesc)]
pub struct LineDestroySystem {
    reader_id: Option<ReaderId<BlockLandEvent>>,
}

impl LineDestroySystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for LineDestroySystem {
    type SystemData = (
        WriteStorage<'s, DeadBlock>,
        WriteStorage<'s, Position>,
        Entities<'s>,
        Write<'s, EventChannel<BlockLandEvent>>,
    );

    fn run(
        &mut self,
        (mut dead_blocks, mut positions, entities, mut land_channel): Self::SystemData,
    ) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for _ in land_channel.read(reader_id) {
            let mut dead_pos_by_row = HashMap::new();
            for (entity, _, dead_position) in (&*entities, &mut dead_blocks, &mut positions).join()
            {
                match dead_pos_by_row.entry(dead_position.row) {
                    Entry::Vacant(e) => {
                        e.insert(vec![(entity, dead_position)]);
                    }
                    Entry::Occupied(mut e) => {
                        e.get_mut().push((entity, dead_position));
                    }
                }
            }

            let mut rows_to_descend: HashMap<i8, i8> = HashMap::new();
            for dead_row in dead_pos_by_row.keys() {
                if let Some(blocks_to_destroy) = dead_pos_by_row.get(dead_row) {
                    if blocks_to_destroy.len() >= BOARD_WIDTH as usize {
                        for block_to_destroy in blocks_to_destroy {
                            entities.delete(block_to_destroy.0).unwrap();
                        }

                        for other_row in dead_pos_by_row.keys().filter(|x| x > &dead_row) {
                            match rows_to_descend.entry(*other_row) {
                                Entry::Vacant(e) => {
                                    e.insert(1);
                                }
                                Entry::Occupied(mut e) => {
                                    *e.get_mut() += 1;
                                }
                            }
                        }
                    }
                }
            }

            for row_to_descend in rows_to_descend {
                if let Some(blocks_to_move) = dead_pos_by_row.get_mut(&row_to_descend.0) {
                    for block_to_move in blocks_to_move {
                        block_to_move.1.row -= row_to_descend.1;
                    }
                }
            }
        }
    }
}
