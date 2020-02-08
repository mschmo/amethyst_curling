extern crate nalgebra as na;

use amethyst::core::{Transform, Time};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, VirtualKeyCode, StringBindings};

use crate::curling::{Stone};


#[derive(SystemDesc, Default)]
pub struct DebugSystem {}


impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Stone>,
        Read<'s, InputHandler<StringBindings>>,
        // Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut stones, input_handler): Self::SystemData) {
        // Press 'r' to reset game
        if input_handler.key_is_down(VirtualKeyCode::R) {
            println!("Reset game");
        }
    }
}
