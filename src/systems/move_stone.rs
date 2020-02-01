extern crate nalgebra as na;

use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use na::Vector3;

use crate::curling::{Stone};


const ACCELERATION: f32 = 0.2;


#[derive(SystemDesc, Default)]
pub struct MoveStoneSystem {}


impl<'s> System<'s> for MoveStoneSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Stone>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut stones, time): Self::SystemData) {
        for (stone, transform) in (&mut stones, &mut transforms).join() {
            let vx = stone.velocity[0] * time.delta_seconds();
            let vy = stone.velocity[1] * time.delta_seconds();
            transform.append_translation(Vector3::new(vx, vy, 0.0));

            stone.velocity[0] = toward_zero(stone.velocity[0]);
            stone.velocity[1] = toward_zero(stone.velocity[1]);
        }
    }
}

fn toward_zero(velocity: f32) -> f32 {
    // Fuck it, optimize later
    match velocity {
        v if v < 0.0 => v + ACCELERATION,
        v if v > 0.0 => v - ACCELERATION,
        _ => 0.0
    }
}
