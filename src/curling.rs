use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
               debug_drawing::{DebugLines, DebugLinesParams}
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

// main game struct
pub struct Curling;

// display.ron => dimensions: Some((450, 800)),
pub const ARENA_WIDTH: f32 = 450. / 2.;
pub const ARENA_HEIGHT: f32 = 800. / 2.;
pub const STONE_RADIUS: f32 = 16. / 2.;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StoneColor {
    Red,
    Blue
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StoneState {
    ReadyToLaunch,
    InPlay,
    Stopped
}

// What properties make sense for a curling stone?
#[derive(Copy, Clone)]
pub struct Stone {
    pub state: StoneState,
    pub color: StoneColor,
    pub radius: f32,
    pub velocity: [f32; 2],
}


#[derive(Default)]
pub struct DebugScreen {
    pub turn_num: u32,
    pub in_play: bool,
    pub is_colliding: bool
}

pub struct DebugText {
    pub turn_num_report: Entity,
    pub player_turn_report: Entity,
    pub in_play_report: Entity,
    pub collision_report: Entity
}

// This allows the app to close
impl SimpleState for Curling {

    // `data` os a structure given to all state methods
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        // Setup debug lines as a resource
        world.insert(DebugLines::new());
        // Configure width of lines. Optional step
        world.insert(DebugLinesParams { line_width: 2.0 });

        // There must be a better way to do this. And there is...
        // Once we add systems, any component that a system operates on will also be registered.
        world.register::<Stone>();
        // TODO: Don't just clone everything
        init_stones(world, sprite_sheet_handle.clone());
        init_target(world, sprite_sheet_handle);
        init_camera(world);
        init_debug_screen(world);
    }

}

impl Stone {
    fn new(color: StoneColor) -> Stone {
        Stone {
            state: StoneState::ReadyToLaunch,
            color,
            radius: STONE_RADIUS,
            velocity: [0.0, 0.0]
        }
    }

    pub fn set_state(&mut self, state: StoneState) {
        self.state = state;
    }

    fn _dbg_new_stopped(color: StoneColor) -> Stone {
        Stone {
            state: StoneState::Stopped,
            color,
            radius: STONE_RADIUS,
            velocity: [0.0, 0.0]
        }
    }
}

impl Component for Stone {
    type Storage = DenseVecStorage<Self>;
}


fn init_camera(world: &mut World) {
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
    transform_blue.set_translation_xyz(ARENA_WIDTH / 2. - 20., ARENA_HEIGHT / 5., 0.);
    transform_red.set_translation_xyz(ARENA_WIDTH / 2. + 20., ARENA_HEIGHT / 5., 0.);

    world
        .create_entity()
        .with(sprite_render_blue.clone())
        .with(Stone::new(StoneColor::Blue))
        .with(transform_blue)
        .build();

    world
        .create_entity()
        .with(sprite_render_red.clone())
        .with(Stone::_dbg_new_stopped(StoneColor::Red))
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

fn _new_ui_transform(report_name: &str, y: f32) -> UiTransform {
     UiTransform::new(
        report_name.to_string(),
        Anchor::Middle, Anchor::Middle,
        -50., y, 1., 200., 50.
    )
}

/// Initialises a ui scoreboard
fn init_debug_screen(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let turn_num_trans = _new_ui_transform("turn_num_report", -50.);
    let player_turn_trans = _new_ui_transform("player_turn_report", -65.);
    let in_play_trans = _new_ui_transform("in_play_report", -80.);
    let collision_trans = _new_ui_transform("collision_report", -95.);
    let turn_num_report = world
        .create_entity()
        .with(turn_num_trans)
        .with(UiText::new(font.clone(), "Turn: 0".to_string(), [0., 0., 0., 1.], 12.,))
        .build();
    let player_turn_report = world
        .create_entity()
        .with(player_turn_trans)
        .with(UiText::new(font.clone(), "Player: Blue".to_string(), [0., 0., 0., 1.], 12.,))
        .build();
    let in_play_report = world
        .create_entity()
        .with(in_play_trans)
        .with(UiText::new(font.clone(), "In Play: False".to_string(), [0., 0., 0., 1.], 12.,))
        .build();
    let collision_report = world
        .create_entity()
        .with(collision_trans)
        .with(UiText::new(font.clone(), "Collision: False".to_string(), [0., 0., 0., 1.], 12.,))
        .build();
    world.insert(DebugText { turn_num_report, player_turn_report, in_play_report, collision_report });
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // `texture_handle` is a cloneable reference to the texture equivalent to a reference-counted option
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
