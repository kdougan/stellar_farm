use components::*;
use hecs::World;
use raylib::prelude::Color;

mod components;
mod systems;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Stellar Farm").build();

    rl.set_target_fps(60);

    let mut world = World::new();

    world.spawn((
        Transform {
            position: glm::vec2(100.0, 100.0),
            size: glm::vec2(16.0, 16.0),
            scale: 1.0,
        },
        Drawable { color: Color::RED },
        Physics {
            velocity: glm::vec2(0.0, 0.0),
            acceleration: glm::vec2(0.0, 0.0),
            mass: 1.0,
        },
        Selected {},
    ));

    while !rl.window_should_close() {
        systems::keyboard_input_system(&mut world, &mut rl);
        systems::mouse_input_system(&mut world, &mut rl);
        systems::move_to_system(&mut world);
        systems::physics_system(&mut world, &mut rl);
        systems::render_system(&world, &mut rl, &thread);
    }
}
