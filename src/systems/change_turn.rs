// Watch for all stones to reach a Stopped state.
// Increment turn. Create new stone.
// Or end game if we've reach all 6 turns.
use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{
        Join, System, SystemData,
        ReadExpect, Write, WriteStorage, Entities, ReadStorage
    },
    renderer::{SpriteRender},
    ui::UiText,
};

use crate::curling::{GameStats, DebugText, Stone, StoneState, StoneColor, CurlingSpriteSheet, Target};


pub const TOTAL_TURNS: u32 = 6;

#[derive(SystemDesc)]
pub struct ChangeTurnSystem;

impl<'s> System<'s> for ChangeTurnSystem {
    type SystemData = (
        WriteStorage<'s, Stone>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, SpriteRender>,
        ReadStorage<'s, Target>,
        Write<'s, GameStats>,
        ReadExpect<'s, DebugText>,
        ReadExpect<'s, CurlingSpriteSheet>,
        Entities<'s>
    );

    fn run(&mut self, (mut stones, mut locals, mut ui_text, mut sprites, targets, mut stats, screen_text, sprite_sheet, entities): Self::SystemData) {
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

            // Calculate the score
            // TODO: BREAK THIS OUT INTO ITS OWN SYSTEM!!!
            stats.score = [0, 0];

            let mut stone_distances: Vec<(StoneColor, f32)> = Vec::new();
            // lol - guess it would be easy to have a multi target mode
            for (t, t_loc) in (&targets, &locals).join() {
                for (s, s_loc) in (&stones, &locals).join() {
                    let x_dist = s_loc.translation().x - t_loc.translation().x;
                    let y_dist = s_loc.translation().y - t_loc.translation().y;
                    // Pythagorean theorem

                    // Rules:
                    // 1. Only stones that are on target are eligible for point
                    // 2. Closest stone to button is the winner and gets 1 point
                    // per stone that is closer than the opponent's closest.
                    match (x_dist.powf(2.0) + y_dist.powf(2.0)).sqrt() {
                        d if d <= t.radius + s.radius => {
                            let color_idx = match s.color {
                                StoneColor::Blue => 0, StoneColor::Red => 1
                            };
                            stats.score[color_idx] += 1;
                            stone_distances.push((s.color, d));
                        },
                        _ => ()
                    };
                }
            }
            stone_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            println!("Stone Distances (Ordered) = {:?}", stone_distances);

            if let Some(text) = ui_text.get_mut(screen_text.score_report) {
                text.text = format!("Red: {} | Blue: {}", stats.score[1].to_string(), stats.score[0].to_string());
            }

            // ok now assign score based on official rules
            let mut score = 0;
            let mut winner: Option<StoneColor> = None;
            for (i, distance_data) in stone_distances.iter().enumerate() {
                if i == 0 {
                    winner = Some(distance_data.0);
                    score += 1;
                    continue;
                }
                match winner {
                    Some(color) => {
                        if color != distance_data.0 {
                            break;
                        }
                        score += 1;
                    },
                    None => break
                }
            }
            println!("The winner is {:?} with a score of {}", winner.unwrap(), score);

            //// --- END SCORE SECTION --- ////

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
