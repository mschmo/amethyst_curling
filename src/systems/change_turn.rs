// Watch for all stones to reach a Stopped state.
// Increment turn. Create new stone.
// Or end game if we've reach all 6 turns.
use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{
        Join, System, SystemData,
        ReadExpect, Write, WriteStorage
    },
    ui::UiText,
};

use crate::curling::{GameStats, DebugText, Stone, StoneState, StoneColor};

#[derive(SystemDesc)]
pub struct ChangeTurnSystem;

impl<'s> System<'s> for ChangeTurnSystem {
    type SystemData = (
        WriteStorage<'s, Stone>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, GameStats>,
        ReadExpect<'s, DebugText>
    );

    fn run(&mut self, (mut stones, mut _locals, mut ui_text, mut stats, screen_text): Self::SystemData) {
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
            stats.in_play = false;
            stats.turn_num += 1;
            for stone in (&mut stones).join() {
                if stats.turn_num % 2 == 0 && stone.color == StoneColor::Blue {
                    stone.set_state(StoneState::ReadyToLaunch);
                    if let Some(text) = ui_text.get_mut(screen_text.player_turn_report) {
                        text.text = "Player: Blue".to_string();
                    }
                } else if stats.turn_num % 2 != 0 && stone.color == StoneColor::Red {
                    stone.set_state(StoneState::ReadyToLaunch);
                    if let Some(text) = ui_text.get_mut(screen_text.player_turn_report) {
                        text.text = "Player: Red".to_string();
                    }
                }
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
