use bevy::prelude::*;
use bevy::utils::HashMap;
// use bevy::log;

use crate::bounds::Bounds2;
use crate::components::coordinates::Coordinates;

use super::map::Map;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: Map,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>
}

impl Board {
    /// Translates a mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let windows_size = Vec2::new(window.width(), window.height());
        let position = Vec2 {
            x: position.x - windows_size.x / 2.,
            y: position.y - windows_size.y / 2.,
        };
        // Bounds check
        if !self.bounds.is_bounds(position) {
            return None;
        }
        // Turn world space to board space
        let coordinates = position - self.bounds.position;
        Some(Coordinates { 
            x: (coordinates.x / self.tile_size) as u16, 
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    /// Retiries a covered tile entity
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        // log::info!("covered_tile: {:?}", self.covered_tiles);
        self.covered_tiles.get(coords)
    }

    /// Trying to uncover a tile
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coords)
    }

    /// Rerieve the adjacent covered tile entities
    pub fn adjacent_covered_tiles(&self, coords: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coords)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    pub fn get_coords(&self, entiry: &Entity) -> Option<&Coordinates> {
        for (co, en) in self.covered_tiles.iter() {
            if en.eq(&entiry) {
                return Some(co);
            }
        }
        return None;
    }
}   