use glam::Vec2;
use raylib::prelude::Color;

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

pub struct MoveTo {
    pub position: Vec2,
}

pub struct Selected {}

pub struct Selectable {}
