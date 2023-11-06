use glam::Vec2;
use hecs::Entity;
use std::collections::HashMap;

const GRID_SIZE: f32 = 100.0;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GridCell {
    x: i32,
    y: i32,
}

pub struct SpatialGrid {
    grid_size: f32,
    cells: HashMap<GridCell, Vec<Entity>>,
}

impl SpatialGrid {
    pub fn new(grid_size: f32) -> Self {
        SpatialGrid {
            grid_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert_one(&mut self, entity: hecs::Entity, position: &Vec2) {
        let cell = self.get_cell_for_position(position);
        self.cells.entry(cell).or_insert_with(Vec::new).push(entity);
    }

    pub fn get_cell_for_position(&self, position: &Vec2) -> GridCell {
        GridCell {
            x: (position.x / self.grid_size).floor() as i32,
            y: (position.y / self.grid_size).floor() as i32,
        }
    }

    pub fn query_nearby(&self, position: &Vec2) -> Vec<Entity> {
        let cell = self.get_cell_for_position(position);
        let mut nearby_entities: Vec<Entity> = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                let neighbor_cell = GridCell {
                    x: cell.x + x,
                    y: cell.y + y,
                };
                if let Some(entities) = self.cells.get(&neighbor_cell) {
                    nearby_entities.extend(entities);
                }
            }
        }
        nearby_entities
    }
}
