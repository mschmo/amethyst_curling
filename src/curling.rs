use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions
};


// display.ron => dimensions: Some((450, 800)),
pub const ARENA_WIDTH: f32 = 450.0 / 2.0;
pub const ARENA_HEIGHT: f32 = 800.0 / 2.0;
// TODO: Use radius instead?
pub const STONE_HEIGHT: f32 = 16.0 / 2.0;
pub const STONE_WIDTH: f32 = 16.0 / 2.0;


// main game struct
pub struct Curling;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StoneColor {
    Red,
    Blue
}

// What properties make sense for a curling stone?
pub struct Stone {
    pub color: StoneColor,
    pub width: f32,
    pub height: f32
}

// This allows the app to close
impl SimpleState for Curling {

    // `data` os a structure given to all state methods
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        // There must be a better way to do this. And there is...
        // Once we add systems, any component that a system operates on will also be registered.
        world.register::<Stone>();
        // TODO: Don't just clone everything
        init_stones(world, sprite_sheet_handle.clone());
        init_target(world, sprite_sheet_handle);
        initialise_camera(world);
    }

}

impl Stone {
    fn new(color: StoneColor) -> Stone {
        Stone {
            color,
            width: STONE_WIDTH,
            height: STONE_HEIGHT
        }
    }
}

impl Component for Stone {
    type Storage = DenseVecStorage<Self>;
}


fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // Bottom left corner is (-width/2.0, -height/2.0)???
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}


fn init_stones(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let sprite_render_blue = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    let sprite_render_red = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    let mut transform_blue = Transform::default();
    let mut transform_red = Transform::default();
    transform_blue.set_translation_xyz(ARENA_WIDTH / 2.0 - 10.0, ARENA_HEIGHT / 6.0, 0.0);
    transform_red.set_translation_xyz(ARENA_WIDTH / 2.0 + 10.0, ARENA_HEIGHT / 6.0, 0.0);

    world
        .create_entity()
        .with(sprite_render_blue.clone())
        .with(Stone::new(StoneColor::Blue))
        .with(transform_blue)
        .build();

    world
        .create_entity()
        .with(sprite_render_red.clone())
        .with(Stone::new(StoneColor::Red))
        .with(transform_red)
        .build();
}

fn init_target(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT - (ARENA_HEIGHT / 5.0), 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    // equivalent to a reference-counted option
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/curling_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/curling_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
