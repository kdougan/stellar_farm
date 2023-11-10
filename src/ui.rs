use hecs::World;
use raylib::prelude::Color;

use crate::{
    components::{Button, TextButton, Transform, UiElement},
    types,
};

struct UIComposer {}

struct UIState {}

impl UIComposer {
    pub fn clear(self, world: &mut World) {
        let ents: Vec<_> = world
            .query::<&UiElement>()
            .iter()
            .map(|(ent, _)| ent)
            .collect::<Vec<_>>();
        for ent in ents {
            let _ = world.despawn(ent);
        }
    }

    pub fn set_state(self, state: UIState) {
        // match state {}
    }
}

pub fn init(world: &mut World) {
    // create a button
    create_button(
        world,
        glam::Vec2::new(10.0, 10.0),
        glam::Vec2::new(100.0, 50.0),
        "Build",
        20.0,
        raylib::prelude::Color::RAYWHITE,
        raylib::prelude::Color::BLACK,
        Some(types::ActionType::EnableBuildMode),
        None,
    );
    create_button(
        world,
        glam::Vec2::new(10.0, 70.0),
        glam::Vec2::new(100.0, 50.0),
        "Select",
        20.0,
        raylib::prelude::Color::RAYWHITE,
        raylib::prelude::Color::BLACK,
        Some(types::ActionType::EnableSelectMode),
        None,
    );
}

pub fn create_button(
    world: &mut World,
    position: glam::Vec2,
    size: glam::Vec2,
    text: &str,
    font_size: f32,
    color: Color,
    bg_color: Color,
    click_action: Option<types::ActionType>,
    hover_action: Option<types::ActionType>,
) {
    world.spawn((
        Transform {
            position,
            size,
            scale: 1.0,
            rotation: 0.0,
        },
        UiElement {},
        TextButton {
            text: text.to_string(),
            font_size,
            color,
            bg_color,
        },
        Button {
            click_action,
            hover_action,
        },
    ));
}
