use amethyst::{
    core::{Transform},
    ecs::prelude::{Join, ReadStorage, System, ReadExpect, Write, WriteStorage},
    ui::UiText
};

use crate::curling::{DebugScreen, DebugText, Stone, StoneColor, ARENA_HEIGHT, ARENA_WIDTH};

fn get_color_id(color: StoneColor) -> f32 {
    match color {
        StoneColor::Blue => 1.0,
        StoneColor::Red => 2.0
    }
}

pub struct CollideStoneSystem;

impl<'s> System<'s> for CollideStoneSystem {
type SystemData = (
    WriteStorage<'s, Stone>,
    ReadStorage<'s, Transform>,
    WriteStorage<'s, UiText>,
    Write<'s, DebugScreen>,
    ReadExpect<'s, DebugText>
);

    fn run(&mut self, (mut stones, transforms, mut ui_text, mut stats, screen_text): Self::SystemData) {
        let mut stone_coords: Vec<[f32; 3]> = Vec::new();
        for (s1, transform) in (&stones, &transforms).join() {
            // println!("{:?} - {:?}", s1.color, transform.translation());
            stone_coords.push([get_color_id(s1.color), transform.translation().x, transform.translation().y]);
        }

        for (s1, transform) in (&mut stones, &transforms).join() {
            let s1_x = transform.translation().x;
            let s1_y = transform.translation().y;

            // Bounce stone at the sides of our arena
            if (s1_y <= s1.radius && s1.velocity[1] < 0.0)  || (s1_y >= ARENA_HEIGHT - s1.radius && s1.velocity[1] > 0.0) {
                s1.velocity[1] = -s1.velocity[1];
            }
            if (s1_x <= s1.radius && s1.velocity[0] < 0.0)  || (s1_x >= ARENA_WIDTH - s1.radius && s1.velocity[0] > 0.0) {
                s1.velocity[0] = -s1.velocity[0];
            }

            // Check collision between stones
            for coords in stone_coords.iter() {
                if get_color_id(s1.color) == coords[0] {
                    continue;
                }
                let s2_x = coords[1];
                let s2_y = coords[2];
                let s1_s2_distance = ((s1_x - s2_x).powf(2.0) + (s1_y - s2_y).powf(2.0)).sqrt();
                // println!("{:?} (s1) distance to coords {:?} = {:?}", s1.color, coords, s1_s2_distance);

                stats.is_colliding = match s1_s2_distance {
                    d if d <= s1.radius * 2.0 => true,
                    _ => false
                };
                if let Some(text) = ui_text.get_mut(screen_text.collision_report) {
                    text.text = format!("Collision: {}", stats.is_colliding.to_string());
                }
            }
        }
    }
}
