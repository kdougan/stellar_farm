use components::*;
use glam::vec2;
use hecs::World;
use raylib::prelude::Color;

mod components;
mod grid;
mod systems;
mod util;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280 / 2, 720 / 2)
        .title("Stellar Farm")
        .build();

    rl.set_target_fps(60);

    let mut world = World::new();

    world.spawn((
        Transform {
            position: vec2(100.0, 100.0),
            size: vec2(16.0, 16.0),
            scale: 1.0,
            rotation: 0.0,
        },
        Drawable { color: Color::RED },
        Physics {
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            mass: 1.0,
        },
        Selected {},
        Selectable {},
    ));

    while !rl.window_should_close() {
        systems::input_system(&mut world, &mut rl);
        systems::move_to_system(&mut world);
        systems::physics_system(&mut world, &mut rl);
        systems::render_system(&world, &mut rl, &thread);
    }
}
