use glam::Vec2;
use raylib::prelude::Color;

use crate::types::{ActionType, BuildingType};

pub struct Transform {
    pub position: Vec2,
    pub size: Vec2,
    pub scale: f32,
    pub rotation: f32,
}

impl Transform {
    pub fn contains_point(&self, point: Vec2) -> bool {
        let top_left = self.position;
        let bottom_right = self.position + self.size;

        point.x >= top_left.x
            && point.x <= bottom_right.x
            && point.y >= top_left.y
            && point.y <= bottom_right.y
    }
}

pub struct Drawable {
    pub color: Color,
}

pub struct Physics {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

pub struct Collider {
    pub radius: f32,
    pub immovable: bool,
}

pub struct MoveTo {
    pub position: Vec2,
}

pub struct Selected {}

pub struct Selectable {}

pub struct BuildProgress {
    pub time_cost: f32,
    pub time_progress: f32,
}

pub struct Building {
    pub building_type: BuildingType,
}

pub struct Health {
    pub health: f32,
    pub max_health: f32,
}

pub struct UiElement {}

pub struct Button {
    pub click_action: Option<ActionType>,
    pub hover_action: Option<ActionType>,
}

pub struct TextButton {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub bg_color: Color,
}
