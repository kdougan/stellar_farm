use crate::components::*;
use hecs::World;
use raylib::{
    consts::MouseButton::MOUSE_LEFT_BUTTON,
    prelude::{Color, RaylibDraw},
    RaylibHandle, RaylibThread,
};

pub fn render_system(world: &World, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    for (_, (transform, drawable)) in &mut world.query::<(&Transform, &Drawable)>() {
        d.draw_circle(
            transform.position.x as i32,
            transform.position.y as i32,
            (transform.size.x * 0.5) * transform.scale,
            drawable.color,
        );
    }
}

pub fn keyboard_input_system(world: &mut World, rl: &mut RaylibHandle) {
    for (_, (physics,)) in &mut world.query::<(&mut Physics,)>() {
        physics.velocity.x = 0.0;
        physics.velocity.y = 0.0;

        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            physics.velocity.y -= 100.0;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            physics.velocity.y += 100.0;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            physics.velocity.x -= 100.0;
        }
        if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
            physics.velocity.x += 100.0;
        }
    }
}

pub fn mouse_input_system(world: &mut World, rl: &mut RaylibHandle) {
    if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
        let vec = rl.get_mouse_position();
        let ids: Vec<_> = world
            .query::<(&Selected,)>()
            .iter()
            .map(|(id, _)| id)
            .collect();
        for id in ids {
            let _ = world.insert_one(
                id,
                MoveTo {
                    position: glm::vec2(vec.x as f32, vec.y as f32),
                },
            );
        }
    }
}

pub fn move_to_system(world: &mut World) {
    let mut to_remove = vec![];
    for (id, (moveto, physics, transform)) in world
        .query::<(&mut MoveTo, &mut Physics, &Transform)>()
        .iter()
    {
        let dx = moveto.position.x - transform.position.x;
        let dy = moveto.position.y - transform.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        if distance > 1.0 {
            physics.velocity.y += (dy / distance) * 100.0;
            physics.velocity.x += (dx / distance) * 100.0;
        } else {
            to_remove.push(id);
        }
    }
    for id in to_remove {
        let _ = world.remove_one::<MoveTo>(id);
    }
}

pub fn physics_system(world: &mut World, rl: &mut RaylibHandle) {
    let dt = rl.get_frame_time();

    for (_, (transform, physics)) in world.query::<(&mut Transform, &Physics)>().iter() {
        transform.position.x += physics.velocity.x * dt;
        transform.position.y += physics.velocity.y * dt;

        let width = rl.get_screen_width() as f32;
        let height = rl.get_screen_height() as f32;

        if transform.position.x < 0.0 {
            transform.position.x = width;
        }
        if transform.position.x > width {
            transform.position.x = 0.0;
        }
        if transform.position.y < 0.0 {
            transform.position.y = height;
        }
        if transform.position.y > height {
            transform.position.y = 0.0;
        }
    }
}
