// Watch for all stones to reach a Stopped state.
// Increment turn. Create new stone.
// Or end game if we've reach all 6 turns.
use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, World, WriteStorage},
};

use crate::curling::{Stone, StoneState};

#[derive(SystemDesc)]
pub struct ChangeTurnSystem;

impl<'s> System<'s> for ChangeTurnSystem {
    type SystemData = (
        WriteStorage<'s, Stone>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut stones, mut locals): Self::SystemData) {
        let mut all_stopped = true;
        for stone in stones.join() {
            if stone.state != StoneState::Stopped {
                all_stopped = false;
                break;
            }
        }
        println!("All stones are stopped = {:?}", all_stopped);
    }
}
