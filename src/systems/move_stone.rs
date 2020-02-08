extern crate nalgebra as na;

use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use na::Vector3;

use crate::curling::{Stone, StoneState};


const DECELERATION: f32 = 0.2;


#[derive(SystemDesc, Default)]
pub struct MoveStoneSystem {}


// TODO: Learn what the <'s> means
impl<'s> System<'s> for MoveStoneSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Stone>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut stones, time): Self::SystemData) {
        for (stone, transform) in (&mut stones, &mut transforms).join() {
            if stone.state != StoneState::InPlay {
                continue;
            }

            let vx = stone.velocity[0] * time.delta_seconds();
            let vy = stone.velocity[1] * time.delta_seconds();
            transform.append_translation(Vector3::new(vx, vy, 0.0));
            stone.velocity[0] = stone.velocity[0] - DECELERATION;
            stone.velocity[1] = stone.velocity[1] - DECELERATION;

            if stone.velocity[0] <= 0.1 && stone.velocity[1] <= 0.1 {
                stone.set_state(StoneState::ReadyToLaunch);
            }
        }
    }
}
