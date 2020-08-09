mod entities;
mod events;
mod states;
mod systems;

use crate::{
    states::GameState,
    systems::{
        BlockFallSystem, BlockInputSystem, BlockSpawnSystem, LineDestroySystem, RenderSystem,
    },
};

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let key_bindings_path = config_dir.join("bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with(
            BlockInputSystem::new(),
            "block_input_system",
            &["input_system"],
        )
        .with(BlockFallSystem::new(), "block_fall_system", &[])
        .with(BlockSpawnSystem::new(), "block_spawn_system", &[])
        .with(LineDestroySystem::new(), "line_destroy_system", &[])
        .with(RenderSystem, "render_system", &[]);

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
