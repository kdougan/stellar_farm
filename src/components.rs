use raylib::prelude::Color;

pub struct Transform {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
    pub scale: f32,
}

pub struct Drawable {
    pub color: Color,
}

pub struct Physics {
    pub velocity: glm::Vec2,
    pub acceleration: glm::Vec2,
    pub mass: f32,
}

pub struct MoveTo {
    pub position: glm::Vec2,
}

pub struct Selected {}
