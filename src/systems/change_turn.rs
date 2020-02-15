// Watch for all stones to reach a Stopped state.
// Increment turn. Create new stone.
// Or end game if we've reach all 6 turns.
use amethyst::{
    assets::{AssetStorage, Handle},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{
        Join, System, SystemData,
        Read, ReadExpect, Write, WriteStorage, Entities, World
    },
    renderer::{SpriteRender, SpriteSheet},
    ui::UiText,
};

use crate::curling::{GameStats, DebugText, Stone, StoneState, StoneColor, CurlingSpriteSheet};


pub const TOTAL_TURNS: u32 = 6;

#[derive(SystemDesc)]
pub struct ChangeTurnSystem;

impl<'s> System<'s> for ChangeTurnSystem {
    type SystemData = (
        WriteStorage<'s, Stone>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, GameStats>,
        ReadExpect<'s, DebugText>,
        ReadExpect<'s, CurlingSpriteSheet>,
        Entities<'s>
    );

    fn run(&mut self, (mut stones, mut locals, mut ui_text, mut sprites, mut stats, screen_text, sprite_sheet, entities): Self::SystemData) {
        let mut all_stopped = true;
        for stone in stones.join() {
            if stone.state == StoneState::ReadyToLaunch {
                // Turn change has already happened, and we are ready to launch
                return;
            }

            if stone.state != StoneState::Stopped {
                all_stopped = false;
                break;
            }
        }
        if all_stopped {
            // All stones have stopped. Next turn...
            stats.in_play = false;
            stats.turn_num += 1;

            if stats.turn_num >= TOTAL_TURNS {
                // Create a blue stone at the starting position
                return
            }

            // 3 total stones each
            // let sprite_sheet = sprite_sheets.get(&sprites.sprite_sheet).unwrap();
            // let sprite = &sprite_sheet.sprites[sprite.sprite_number];
            if stats.turn_num % 2 == 0 {
                if let Some(text) = ui_text.get_mut(screen_text.player_turn_report) {
                    text.text = "Player: Blue".to_string();
                }
                let sr = SpriteRender { sprite_sheet: Handle::from(sprite_sheet.handle.clone()), sprite_number: 0};
                entities.build_entity()
                    .with(sr.clone(), &mut sprites)
                    .with(Stone::new(StoneColor::Blue), &mut stones)
                    .with(Stone::get_starting_pos(), &mut locals)
                    .build();
            } else {
                if let Some(text) = ui_text.get_mut(screen_text.player_turn_report) {
                    text.text = "Player: Red".to_string();
                }
                let sr = SpriteRender { sprite_sheet: sprite_sheet.handle.clone(), sprite_number: 1};
                entities.build_entity()
                    .with(sr.clone(), &mut sprites)
                    .with(Stone::new(StoneColor::Red), &mut stones)
                    .with(Stone::get_starting_pos(), &mut locals)
                    .build();
            }
        } else {
            stats.in_play = true;
        }

        if let Some(text) = ui_text.get_mut(screen_text.turn_num_report) {
            text.text = format!("Turn: {}", stats.turn_num.to_string());
        }
        if let Some(text) = ui_text.get_mut(screen_text.in_play_report) {
            text.text = format!("In Play: {}", stats.in_play.to_string());
        }
    }
}
