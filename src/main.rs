use actions::ActionQueue;
use grid::SpatialGrid;
use hecs::World;

mod actions;
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
    let mut spatial_grid = SpatialGrid::new(64.0);
    let mut action_queue = ActionQueue::new();

    init(&mut world);

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

fn init(world: &mut World) {
    // create a button
    world.spawn((
        components::Transform {
            position: glam::Vec2::new(60.0, 30.0),
            size: glam::Vec2::new(100.0, 50.0),
            scale: 1.0,
            rotation: 0.0,
        },
        components::UiElement { visible: true },
        components::TextButton {
            text: "Button".to_string(),
            font_size: 20.0,
            color: raylib::prelude::Color::RAYWHITE,
            bg_color: raylib::prelude::Color::BLACK,
        },
        components::Button {
            click_action: Some(types::ActionType::EnableBuildMode),
            hover_action: None,
        },
    ));
}
