use crate::{
    entities::{Block, BlockType, Position, BOARD_HEIGHT, BOARD_WIDTH},
    events::BlockLandEvent,
};

use amethyst::renderer::{SpriteSheet, SpriteSheetFormat};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        camera::Camera, debug_drawing::DebugLinesComponent, formats::texture::ImageFormat, Texture,
    },
    shrev::EventChannel,
};

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

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

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "tetris_block.png",
                ImageFormat::default(),
                (),
                &world.read_resource::<AssetStorage<Texture>>(),
            )
        };

        let spritesheet_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "sprites.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &world.read_resource::<AssetStorage<SpriteSheet>>(),
            )
        };

        world.insert(spritesheet_handle);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
