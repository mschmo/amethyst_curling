//! Curling!!!
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod curling;
use crate::curling::Curling;


fn main() -> amethyst::Result<()> {
    // We'll put the rest of the code here.
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

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
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let mut world = World::new();
    // root object of the game engine
    // within this object is everything needed to run the game
    let mut game = Application::new(assets_dir, Curling, game_data)?;

    // runs until SimpleState returns Trans::Quit
    // or when all states have been pushed off of the state machine's stack
    game.run();
    Ok(())
}
