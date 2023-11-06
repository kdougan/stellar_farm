use hecs::World;

mod components;
mod grid;
mod systems;
mod types;
mod util;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280 / 2, 720 / 2)
        .title("Stellar Farm")
        .build();

    rl.set_target_fps(60);

    let mut world = World::new();
    let mut spatial_grid = grid::SpatialGrid::new(64.0);

    while !rl.window_should_close() {
        systems::input_system(&mut world, &mut rl);
        systems::move_to_system(&mut world);
        systems::physics_system(&mut world, &mut rl);
        systems::spatial_grid_system(&mut world, &mut spatial_grid);
        systems::separate_entities_system(&mut world, &mut spatial_grid);
        systems::building_progress_system(&mut world, &rl);
        systems::render_system(&world, &mut rl, &thread);
    }
}
