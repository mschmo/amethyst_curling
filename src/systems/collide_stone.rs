use std::collections::HashMap;

use amethyst::{
    core::{Transform},
    ecs::prelude::{Join, ReadStorage, System, WriteStorage, Entities}
};

use crate::curling::{Stone, StoneColor, ARENA_HEIGHT, ARENA_WIDTH, StoneState};


pub struct CollideStoneSystem;

impl<'s> System<'s> for CollideStoneSystem {
    type SystemData = (
        WriteStorage<'s, Stone>,
        ReadStorage<'s, Transform>,
        Entities<'s>
    );

    fn run(&mut self, (mut stones, transforms, entities): Self::SystemData) {
        // {id: [[x, y], [vx, vy]]}
        let mut stone_coords: HashMap<u32, [[f32; 2]; 2]> = HashMap::new();
        for (stone, transform, entity) in (&stones, &transforms, &entities).join() {
            // println!("{:?} - {:?}", s1.color, transform.translation());
            stone_coords.insert(entity.id(),
                                [[transform.translation().x, transform.translation().y], stone.velocity]);
        }

        for (s1, transform, entity) in (&mut stones, &transforms, &entities).join() {
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
            for (entity_id, opp_stone) in &stone_coords {
                if &entity.id() == entity_id {
                    continue;
                }
                let s2_x = opp_stone[0][0];
                let s2_y = opp_stone[0][1];
                let s1_s2_distance = ((s1_x - s2_x).powf(2.0) + (s1_y - s2_y).powf(2.0)).sqrt();
                // println!("{:?} (s1) distance to coords {:?} = {:?}", s1.color, coords, s1_s2_distance);

                let is_colliding = match s1_s2_distance {
                    d if d <= s1.radius * 2.0 => true,  _ => false
                };

                if is_colliding {
                    // Elastic collision
                    // TODO: Don't always assume mass to be equal
                    s1.velocity[0] = opp_stone[1][0];
                    s1.velocity[1] = opp_stone[1][1];
                    println!("Setting {:?} v = {:?}", s1.color, opp_stone[1]);
                    if s1.velocity == [0. , 0.] {
                        s1.set_state(StoneState::Stopped);
                    } else {
                        s1.set_state(StoneState::InPlay);
                    }
                }
            }
        }
    }
}
