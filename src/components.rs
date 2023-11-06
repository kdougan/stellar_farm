use glam::Vec2;
use raylib::prelude::Color;

use crate::types::BuildingType;

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
