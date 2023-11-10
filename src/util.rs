use crate::components::Transform;
use glam::Vec2;

pub fn transform_contains_point(transform: &Transform, point: Vec2) -> bool {
    let top_left = transform.position;
    let bottom_right = transform.position + transform.size;

    point.x >= top_left.x
        && point.x <= bottom_right.x
        && point.y >= top_left.y
        && point.y <= bottom_right.y
}
