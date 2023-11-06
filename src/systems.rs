use crate::{components::*, grid::SpatialGrid};
use glam::{vec2, Vec2};
use hecs::{Entity, World};
use raylib::{
    consts::KeyboardKey::KEY_B,
    consts::KeyboardKey::KEY_SPACE,
    consts::MouseButton::MOUSE_LEFT_BUTTON,
    consts::MouseButton::MOUSE_RIGHT_BUTTON,
    prelude::{Color, RaylibDraw},
    RaylibHandle, RaylibThread,
};

pub fn render_system(world: &World, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::new(30, 20, 30, 255));

    for (_, (transform, moveto, _)) in &mut world.query::<(&Transform, &MoveTo, &Selected)>() {
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
    let mpos_vec = vec2(mpos.x, mpos.y);
    if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
        // Deselect all
        let entities: Vec<_> = world
            .query::<(&Selected,)>()
            .iter()
            .map(|(id, _)| id)
            .collect();
        for ent in entities {
            let _ = world.remove_one::<Selected>(ent);
        }
        // Select new
        let entities: Vec<_> = world
            .query::<(&Transform, &Selectable)>()
            .iter()
            .filter(|(_, (transform, _))| {
                transform.position.distance(mpos_vec) <= (transform.size.x * 0.5) * transform.scale
            })
            .map(|(id, _)| id)
            .collect();
        for ent in entities {
            let _ = world.insert_one(ent, Selected {});
        }
    } else if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
        let entities: Vec<_> = world
            .query::<(&Transform, &Physics, &Selected)>()
            .iter()
            .map(|(id, _)| id)
            .collect();
        for ent in entities {
            let _ = world.insert_one(ent, MoveTo { position: mpos_vec });
        }
    }

    if rl.is_key_pressed(KEY_SPACE) {
        world.spawn((
            Transform {
                position: mpos_vec,
                size: vec2(16.0, 16.0),
                scale: 1.0,
                rotation: 0.0,
            },
            Drawable { color: Color::RED },
            Physics {
                velocity: vec2(0.0, 0.0),
                acceleration: vec2(0.0, 0.0),
                mass: 1.0,
            },
            Collider {
                radius: 8.0,
                immovable: false,
            },
            Selectable {},
        ));
    }
    if rl.is_key_pressed(KEY_B) {
        world.spawn((
            Transform {
                position: mpos_vec,
                size: vec2(32.0, 32.0),
                scale: 1.0,
                rotation: 0.0,
            },
            Drawable { color: Color::BLUE },
            Physics {
                velocity: vec2(0.0, 0.0),
                acceleration: vec2(0.0, 0.0),
                mass: 1.0,
            },
            Collider {
                radius: 8.0,
                immovable: true,
            },
        ));
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

pub fn spatial_grid_system(world: &mut World, spatial_grid: &mut SpatialGrid) {
    spatial_grid.clear();
    for (id, (transform,)) in world.query::<(&Transform,)>().iter() {
        spatial_grid.insert_one(id, &transform.position);
    }
}

pub fn separate_entities_system(world: &mut World, spatial_grid: &mut SpatialGrid) {
    // We will store the movements we need to apply in a Vec and apply them after iterating through entities
    let mut movements: Vec<(Entity, Vec2)> = Vec::new();

    for (entity, (transform, collider, physics)) in
        world.query::<(&Transform, &Collider, &Physics)>().iter()
    {
        let nearby_entities = spatial_grid.query_nearby(&transform.position);

        for other_entity in nearby_entities {
            if other_entity == entity {
                continue;
            }

            let mut query = world
                .query_one::<(&Transform, &Collider, &Physics)>(other_entity)
                .unwrap();
            if let Some((other_transform, other_collider, other_physics)) = query.get() {
                let delta = transform.position - other_transform.position;
                let distance = delta.length();
                let combined_radius = collider.radius + other_collider.radius;

                if distance < combined_radius {
                    let overlap = combined_radius - distance;
                    let direction = if distance != 0.0 {
                        delta / distance
                    } else {
                        vec2(1.0, 0.0)
                    };

                    let total_mass = if collider.immovable {
                        physics.mass
                    } else if other_collider.immovable {
                        other_physics.mass
                    } else {
                        physics.mass + other_physics.mass
                    };

                    let move_entity = if collider.immovable {
                        0.0
                    } else {
                        other_physics.mass / total_mass
                    };

                    let move_other_entity = if other_collider.immovable {
                        0.0
                    } else {
                        physics.mass / total_mass
                    };

                    let entity_movement = direction * (overlap * move_entity);
                    let other_entity_movement = -direction * (overlap * move_other_entity);

                    if !collider.immovable {
                        movements.push((entity, entity_movement));
                    }
                    if !other_collider.immovable {
                        movements.push((other_entity, other_entity_movement));
                    }
                }
            }
        }
    }

    // Apply the movements
    for (entity, movement) in movements {
        if let Ok(mut transform) = world.get::<&mut Transform>(entity) {
            transform.position += movement;
        }
    }
}

pub fn physics_system(world: &mut World, rl: &RaylibHandle) {
    let dt = rl.get_frame_time();
    let screen_dim = vec2(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
    for (_, (transform, physics)) in world.query::<(&mut Transform, &mut Physics)>().iter() {
        physics.velocity += physics.acceleration * dt;
        transform.position += physics.velocity * dt;
        physics.acceleration = vec2(0.0, 0.0);
        physics.velocity *= 0.8;
        transform.position = (transform.position + screen_dim) % screen_dim;
    }
}

pub fn building_progress_system(world: &mut World, rl: &RaylibHandle) {
    let dt = rl.get_frame_time();
    let mut to_remove = vec![];
    world
        .query::<(&mut BuildProgress, &mut Building)>()
        .iter()
        .for_each(|(id, (build_location, _))| {
            build_location.time_progress += dt;
            if build_location.time_progress >= build_location.time_cost {
                to_remove.push(id);
            }
        });
    for id in to_remove {
        let _ = world.remove_one::<BuildProgress>(id);
    }
}
