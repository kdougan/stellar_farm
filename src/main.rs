use actions::ActionQueue;
use grid::SpatialGrid;
use hecs::World;

mod actions;
mod components;
mod grid;
mod modes;
mod systems;
mod types;
mod ui;
mod util;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280 / 2, 720 / 2)
        .title("Stellar Farm")
        .build();

    rl.set_target_fps(60);

    let mut world = World::new();
    let mut spatial_grid = SpatialGrid::new(64.0);
    let mut action_queue = ActionQueue::new();

    ui::init(&mut world);

    while !rl.window_should_close() {
        systems::input_ui_system(&mut world, &mut rl, &mut action_queue);
        systems::input_system(&mut world, &mut rl);
        systems::action_processing_system(&mut world, &mut action_queue);
        systems::move_to_system(&mut world);
        systems::physics_system(&mut world, &mut rl);
        systems::spatial_grid_system(&mut world, &mut spatial_grid);
        systems::separate_entities_system(&mut world, &mut spatial_grid);
        systems::building_progress_system(&mut world, &rl);

        let mut d = rl.begin_drawing(&thread);
        systems::render_system(&world, &mut d);
        systems::render_ui_system(&world, &mut d);
    }
}
