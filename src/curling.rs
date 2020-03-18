use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, VecStorage, Entity},
    input::{VirtualKeyCode, is_key_down},
    prelude::*,
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
        debug_drawing::{DebugLines, DebugLinesParams}
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
    utils::removal::{exec_removal, Removal}
};

// main game struct
#[derive(Default)]
pub struct Curling;

// display.ron => dimensions: Some((450, 800)),
pub const ARENA_WIDTH: f32 = 450. / 2.;
pub const ARENA_HEIGHT: f32 = 800. / 2.;
pub const STONE_RADIUS: f32 = 16. / 2.;
pub const TARGET_RADIUS: f32 = 128. / 2.;
// TODO: Think this would be interesting if it were adjustable
pub const STONE_MASS: f32 = 40.;


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
#[derive(Copy, Clone, Debug)]
pub struct Stone {
    pub state: StoneState,
    pub color: StoneColor,
    pub radius: f32,
    pub velocity: [f32; 2],
    pub mass: f32
}

impl Component for Stone {
    type Storage = VecStorage<Self>;
}

impl Stone {
    pub fn new(color: StoneColor) -> Stone {
        Stone {
            state: StoneState::ReadyToLaunch,
            color,
            radius: STONE_RADIUS,
            velocity: [0.0, 0.0],
            mass: STONE_MASS
        }
    }

    pub fn set_state(&mut self, state: StoneState) {
        self.state = state;
    }

    pub fn get_starting_pos() -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH / 2., ARENA_HEIGHT / 5., 0.5);
        return transform;
    }

    fn _dbg_new_stopped(color: StoneColor) -> Stone {
        Stone {
            state: StoneState::Stopped,
            color,
            radius: STONE_RADIUS,
            velocity: [0.0, 0.0],
            mass: STONE_MASS
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Target {
    pub radius: f32
}

impl Component for Target {
    type Storage = VecStorage<Self>;
}

impl Target {
    fn new() -> Target {
        Target {
            radius: TARGET_RADIUS
        }
    }
}

#[derive(Default)]
pub struct GameStats {
    pub turn_num: u32,
    pub in_play: bool,
    /// [<Blue Score>, <Red Score>]
    pub score: [u32; 2],
    pub game_is_over: bool,
    pub winner: Option<StoneColor>,
    pub winner_score: u32,
    pub please_stop: bool,
}

impl GameStats {
    pub fn reset(&mut self) {
        self.turn_num = 0;
        self.in_play = false;
        self.score = [0, 0];
        self.game_is_over = false;
        self.winner = None;
        self.winner_score = 0;
        self.please_stop = false;
    }
}

pub struct DebugText {
    pub turn_num_report: Entity,
    pub player_turn_report: Entity,
    pub in_play_report: Entity,
    pub score_report: Entity
}

/// Resource to hold the loaded sprite sheet
pub struct CurlingSpriteSheet {
    pub handle: Handle<SpriteSheet>
}

impl CurlingSpriteSheet {
    pub fn new(handle: Handle<SpriteSheet>) -> CurlingSpriteSheet {
        CurlingSpriteSheet { handle }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RemovalId {
    GamePlayEntity,
    Other
}

// This allows the app to close
impl SimpleState for Curling {
    // `data` os a structure given to all state methods
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        world.insert(CurlingSpriteSheet::new(sprite_sheet_handle.clone()));

        // Setup debug lines as a resource
        world.insert(DebugLines::new());
        // Configure width of lines. Optional step
        world.insert(DebugLinesParams { line_width: 2.0 });

        world.register::<Removal<RemovalId>>();

        // TODO: Don't just clone everything
        self._place_stone(world, StoneColor::Blue, sprite_sheet_handle.clone());
        self._init_target(world, sprite_sheet_handle.clone());
        init_camera(world);
        init_debug_screen(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let mut stats = data.world.write_resource::<GameStats>();
        if !stats.game_is_over || stats.please_stop {
            return Trans::None;
        }
        match stats.winner {
            Some(w) => println!("The winner is {:?} with a score of {}", w, stats.winner_score),
            _ => println!("Nobody wins")
        }
        stats.please_stop = true;

        // TODO: Get restart working properly withing spazzing and creating entities left and right
//        let sprite_sheet_handle = load_sprite_sheet(&mut data.world);
//        {
//            let mut stats = data.world.write_resource::<GameStats>();
//            // TODO: Implement a pause state before resetting everything
//            exec_removal(
//                &data.world.entities(),
//                &data.world.read_storage(),
//                RemovalId::GamePlayEntity
//            );
//            stats.reset();
//        }
//
//        let sprite_render = SpriteRender {
//            sprite_sheet: sprite_sheet_handle.clone(),
//            sprite_number: 0,
//        };
//        &data.world
//            .create_entity()
//            .with(sprite_render.clone())
//            .with(Stone::new(StoneColor::Blue))
//            .with(Stone::get_starting_pos())
//            .with(Removal::new(RemovalId::GamePlayEntity))
//            .build();

        Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        println!("Curling game has stopped!");
    }

    // let's add en event handle for the escape button.
    // This is just to test state transitions, but pressing esc should restart the game
    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        // The bug here is this does not seem to clear the existing state
        // it just adds the new game state on top?
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Replace(Box::new(Curling));
            }
        }
        Trans::None
    }
}


impl Curling {

    pub fn _place_stone(&self, world: &mut World, color: StoneColor, sprite_sheet_handle: Handle<SpriteSheet>) {
        let sprite_number = match color {
            StoneColor::Blue => 0,
            StoneColor::Red => 1
        };
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number,
        };

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Stone::new(color))
            .with(Stone::get_starting_pos())
            .with(Removal::new(RemovalId::GamePlayEntity))
            .build();
    }

    fn _init_target(&self, world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 2,
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT - (ARENA_HEIGHT / 5.0), 0.0);
        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Target::new())
            .with(transform)
            .build();
    }
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

fn _new_ui_transform(report_name: &str, y: f32) -> UiTransform {
     UiTransform::new(
        report_name.to_string(),
        Anchor::MiddleLeft, Anchor::MiddleLeft,
        -20., y, 1., 300., 50.
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
    let turn_num_trans = _new_ui_transform("turn_num_report", -270.);
    let player_turn_trans = _new_ui_transform("player_turn_report", -300.);
    let in_play_trans = _new_ui_transform("in_play_report", -330.);
    let score_trans = _new_ui_transform("score_report", -360.);
    let turn_num_report = world
        .create_entity()
        .with(turn_num_trans)
        .with(UiText::new(font.clone(), "Turn: 0".to_string(), [0., 0., 0., 1.], 24.,))
        .build();
    let player_turn_report = world
        .create_entity()
        .with(player_turn_trans)
        .with(UiText::new(font.clone(), "Player: Blue".to_string(), [0., 0., 0., 1.], 24.,))
        .build();
    let in_play_report = world
        .create_entity()
        .with(in_play_trans)
        .with(UiText::new(font.clone(), "In Play: False".to_string(), [0., 0., 0., 1.], 24.,))
        .build();
    let score_report = world
        .create_entity()
        .with(score_trans)
        .with(UiText::new(font.clone(), "Red: 0 | Blue: 0".to_string(), [0., 0., 0., 1.], 24.,))
        .build();
    world.insert(DebugText { turn_num_report, player_turn_report, in_play_report, score_report});
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
