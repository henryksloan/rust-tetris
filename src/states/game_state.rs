use crate::{
    entities::{Block, BlockType, Position, BOARD_HEIGHT, BOARD_WIDTH},
    events::BlockLandEvent,
};

use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{camera::Camera, debug_drawing::DebugLinesComponent},
    shrev::EventChannel,
};

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // world.insert(Block::new(BlockType::S));
        let mut b = Block::new(BlockType::I);
        b.rotation = 3;
        world
            .create_entity()
            .with(b)
            .with(Position { row: 11, col: 3 })
            .build();

        // Setup debug lines as a component and add lines to render axes & grid
        let debug_lines_component = DebugLinesComponent::new();
        world.register::<DebugLinesComponent>();
        world.create_entity().with(debug_lines_component).build();

        let mut land_channel = EventChannel::<BlockLandEvent>::new();
        land_channel.single_write(BlockLandEvent {});
        world.insert(land_channel);

        let mut transform = Transform::default();
        transform.set_translation_xyz(BOARD_WIDTH as f32 * 0.5, BOARD_HEIGHT as f32 * 0.5, 1.0);
        world
            .create_entity()
            .with(Camera::standard_2d(BOARD_WIDTH as f32, BOARD_HEIGHT as f32))
            .with(transform)
            .build();
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
