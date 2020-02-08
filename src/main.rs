//! Curling!!!
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow, RenderDebugLines},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod curling;
mod systems;

use crate::curling::Curling;


fn main() -> amethyst::Result<()> {
    // We'll put the rest of the code here.
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config").join("display.ron");

    // Using StringBindings to identify axes like "stone"
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    // central repo for game logic
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 1.0, 1.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default())
                // TODO: Trying to use this to draw the line towards the mouse cursor when charging
                // but I can't get it to work yet :(
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(systems::LaunchStoneSystem::default(), "launch_stone_system", &["input_system"])
        .with(systems::MoveStoneSystem::default(), "move_stone_system", &["launch_stone_system"])
        .with(systems::CollideStoneSystem, "collide_stone_system", &["launch_stone_system"])
        .with(systems::ChangeTurnSystem, "change_turn_system", &["launch_stone_system"]);



    // root object of the game engine
    // within this object is everything needed to run the game
    let mut game = Application::new(assets_dir, Curling, game_data)?;

    // runs until SimpleState returns Trans::Quit
    // or when all states have been pushed off of the state machine's stack
    game.run();
    Ok(())
}
