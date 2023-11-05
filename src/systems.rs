use crate::components::*;
use glam::vec2;
use hecs::World;
use raylib::{
    consts::MouseButton::MOUSE_LEFT_BUTTON,
    consts::MouseButton::MOUSE_RIGHT_BUTTON,
    prelude::{Color, RaylibDraw},
    RaylibHandle, RaylibThread,
};

pub fn render_system(world: &World, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::new(30, 20, 30, 255));

    for (_, (transform, moveto)) in &mut world.query::<(&Transform, &MoveTo)>() {
        d.draw_line(
            transform.position.x as i32,
            transform.position.y as i32,
            moveto.position.x as i32,
            moveto.position.y as i32,
            Color::SKYBLUE,
        );
        d.draw_circle_lines(
            moveto.position.x as i32,
            moveto.position.y as i32,
            8.0,
            Color::SKYBLUE,
        );
    }
    for (_, (transform, _)) in &mut world.query::<(&Transform, &Selected)>() {
        d.draw_circle_lines(
            transform.position.x as i32,
            transform.position.y as i32,
            (transform.size.x * 0.5 + 4.0) * transform.scale,
            Color::GREEN,
        );
    }
    for (_, (transform, drawable)) in &mut world.query::<(&Transform, &Drawable)>() {
        d.draw_circle(
            transform.position.x as i32,
            transform.position.y as i32,
            (transform.size.x * 0.5) * transform.scale,
            drawable.color,
        );
    }
}

pub fn input_system(world: &mut World, rl: &mut RaylibHandle) {
    let mpos = rl.get_mouse_position();
    let mpos_vec = vec2(mpos.x as f32, mpos.y as f32);
    if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
        // Deselect all
        let ids: Vec<_> = world
            .query::<(&Selected,)>()
            .iter()
            .map(|(id, _)| id)
            .collect();
        for id in ids {
            let _ = world.remove_one::<Selected>(id);
        }
        // Select new
        let ids: Vec<_> = world
            .query::<(&Transform, &Selectable)>()
            .iter()
            .filter(|(_, (transform, _))| {
                transform.position.distance(mpos_vec) <= (transform.size.x * 0.5) * transform.scale
            })
            .map(|(id, _)| id)
            .collect();
        for id in ids {
            let _ = world.insert_one(id, Selected {});
        }
    } else if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
        let ids: Vec<_> = world
            .query::<(&Transform, &Physics, &Selected)>()
            .iter()
            .map(|(id, _)| id)
            .collect();
        for id in ids {
            let _ = world.insert_one(id, MoveTo { position: mpos_vec });
        }
    }
}

pub fn move_to_system(world: &mut World) {
    let mut to_remove = vec![];
    for (id, (moveto, physics, transform)) in world
        .query::<(&mut MoveTo, &mut Physics, &Transform)>()
        .iter()
    {
        let vector = moveto.position - transform.position;
        let distance_squared = vector.length_squared();
        if moveto.position.distance(transform.position) <= 4.0 {
            to_remove.push(id);
        } else {
            let direction = vector / distance_squared.sqrt();
            physics.acceleration = direction * (1000.0 / physics.mass);
        }
    }
    for id in to_remove {
        let _ = world.remove_one::<MoveTo>(id);
    }
}

pub fn physics_system(world: &mut World, rl: &RaylibHandle) {
    let dt = rl.get_frame_time();
    let screen_width = rl.get_screen_width() as f32;
    let screen_height = rl.get_screen_height() as f32;
    for (_, (transform, physics)) in world.query::<(&mut Transform, &mut Physics)>().iter() {
        physics.velocity += physics.acceleration * dt;
        transform.position += physics.velocity * dt;
        physics.acceleration = vec2(0.0, 0.0);
        physics.velocity *= 0.8;
        transform.position.x = (transform.position.x + screen_width) % screen_width;
        transform.position.y = (transform.position.y + screen_height) % screen_height;
    }
}
