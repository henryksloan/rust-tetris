use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{ReaderId, System, SystemData, Write, WriteStorage},
        Entities,
    },
    shrev::EventChannel,
};

use crate::{
    entities::{Block, Position},
    events::BlockLandEvent,
};

#[derive(SystemDesc)]
pub struct BlockSpawnSystem {
    reader_id: Option<ReaderId<BlockLandEvent>>,
}

impl BlockSpawnSystem {
    pub fn new() -> Self {
        Self { reader_id: None }
    }
}

impl<'s> System<'s> for BlockSpawnSystem {
    type SystemData = (
        WriteStorage<'s, Block>,
        Write<'s, EventChannel<BlockLandEvent>>,
        WriteStorage<'s, Position>,
        Entities<'s>,
    );

    fn run(&mut self, (mut blocks, mut land_channel, mut positions, entities): Self::SystemData) {
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| land_channel.register_reader());

        for event in land_channel.read(reader_id) {
            println!("Received an event: {:?}", event);
            let mut b = Block::new(rand::random());
            b.rotation = 0;
            entities
                .build_entity()
                .with(b, &mut blocks)
                .with(Position { row: 11, col: 3 }, &mut positions)
                .build();
        }
    }
}
