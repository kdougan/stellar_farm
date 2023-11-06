use glam::Vec2;
use hecs::Entity;
use raylib::prelude::{Color, Rectangle};

use crate::types::{ActionType, BuildingType};

pub struct Transform {
    pub position: Vec2,
    pub size: Vec2,
    pub scale: f32,
    pub rotation: f32,
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

pub struct UiElement {
    pub bounds: Rectangle,
    pub visible: bool,
}

pub struct Button {
    pub text: String,
    pub action: ActionType,
}

pub struct UiState {
    pub hovered_entity: Option<Entity>,
    pub clicked_entity: Option<Entity>,
}
