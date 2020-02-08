extern crate nalgebra as na;

use amethyst::core::{Transform};
use amethyst::core::geometry::{Plane};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{
    Join, System, SystemData,  // traits
    Read, ReadStorage, WriteStorage, ReadExpect, Entities, Write  // structs
};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::debug_drawing::DebugLines;
use amethyst::renderer::camera::{ActiveCamera, Camera};
use amethyst::renderer::palette::Srgba;
use amethyst::window::ScreenDimensions;
use na::{Vector2};
use na::geometry::{Point3};

use crate::curling::{Stone, StoneState};


const MAX_LAUNCH_VELOCITY: f32 = 100.0;
const LAUNCH_INCREMENT: f32 = 1.0;

#[derive(SystemDesc, Default)]
pub struct LaunchStoneSystem {
    launch_velocity: f32,
    is_charging: bool,
}

// How to determine the right "systems"?
impl<'s> System<'s> for LaunchStoneSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Stone>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ActiveCamera>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        Write<'s, DebugLines>,
        Entities<'s>,
    );

    fn run(&mut self, (transforms, mut stones, input, active_camera, cameras, dimensions, mut debug_lines_resource, entities): Self::SystemData) {
        let mut is_stone_ready_for_launch = false;
        for stone in stones.join() {
            if stone.state == StoneState::ReadyToLaunch {
                is_stone_ready_for_launch = true;
                break
            }
        }
        if !is_stone_ready_for_launch {
            return;
        }

        if let Some(mouse_position) = input.mouse_position() {
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera.entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // TODO: Should this input be an action or axes?
                let action_down = input.action_is_down("launch_stone").expect("selection action missing");
                if action_down && !self.is_charging {
                    self.is_charging = true;

                } else if action_down && self.is_charging {
                    if self.launch_velocity < MAX_LAUNCH_VELOCITY {
                        self.launch_velocity += LAUNCH_INCREMENT;
                    }
                    let pct_charged = self.launch_velocity / MAX_LAUNCH_VELOCITY;
                    for (_, transform) in (&stones, &transforms).join() {
                        // TODO: Consolidate this with code from "launch" section
                        // Draw the "power" line
                        let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());
                        let end_coordinate = Point3::new(mouse_position.0, mouse_position.1, camera_transform.translation().z);
                        let end_world = camera.projection().screen_to_world_point(end_coordinate, screen_dimensions, camera_transform);

                        let end_x = transform.translation().x + ((end_world.coords.x - transform.translation().x) * pct_charged);
                        let end_y = transform.translation().y + ((end_world.coords.y - transform.translation().y) * pct_charged);
                        debug_lines_resource.draw_line(
                            [transform.translation().x, transform.translation().y, 0.5].into(),
                            [end_x, end_y, 0.5].into(),
                            Srgba::new(1.0, 0.0, 0.2, 1.0),
                        );
                    }
                } else if !action_down && self.is_charging {
                    // Launch!
                    let start_coordinate = Some(Point3::new(
                        mouse_position.0,
                        mouse_position.1,
                        camera_transform.translation().z,
                    ));

                    let screen_dimensions = Vector2::new(dimensions.width(), dimensions.height());
                    let end_coordinate = Point3::new(
                        mouse_position.0,
                        mouse_position.1,
                        camera_transform.translation().z,
                    );

                    let start_world = camera.projection().screen_to_world_point(
                        start_coordinate.expect("Wut?"),
                        screen_dimensions,
                        camera_transform,
                    );
                    let end_world = camera.projection().screen_to_world_point(
                        end_coordinate,
                        screen_dimensions,
                        camera_transform,
                    );
                    let plane = Plane::with_z(0.0);
                    let start_world_plane = camera
                        .projection()
                        .screen_ray(
                            start_coordinate.expect("Wut?").xy(),
                            screen_dimensions,
                            camera_transform,
                        )
                        .intersect_plane(&plane);
                    let end_world_plane = camera
                        .projection()
                        .screen_ray(end_coordinate.xy(), screen_dimensions, camera_transform)
                        .intersect_plane(&plane);

                    for (stone, transform) in (&mut stones, &transforms).join() {
                        // https://en.wikipedia.org/wiki/Atan2
                        // atan2(y2 - y1, x2 - x1)
                        let a = (end_world.coords.y - transform.translation().y).atan2(end_world.coords.x - transform.translation().x);
                        stone.velocity[0] = a.cos() * self.launch_velocity;
                        stone.velocity[1] = a.sin() * self.launch_velocity;
                        stone.set_state(StoneState::InPlay);
                        // println!("Launch {:?} stone at initial velocity = {:?}", stone.color, stone.velocity);
                    }

                    self.is_charging = false;
                    self.launch_velocity = 0.0;
                }
            }
        }
    }
}
