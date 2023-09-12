use std::ops::{Deref, DerefMut};
use rand::{thread_rng, Rng};

use super::tile::Tile;
use crate::components::coordinates::Coordinates;

// Delta coordinates for all 8 square neighbors
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug)]
pub struct Map {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>
}

impl Map {
    /// Generates a new map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }
    
    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)    
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }
        let res = self
            .safe_square_at(coordinates)
            .filter(|coor| self.is_bomb_at(*coor))
            .count();
        res as u8
    }

    pub fn set_bombs(&mut self, boom_count: u16) {
        self.bomb_count = boom_count;
        let mut remaining_bombs = boom_count;
        let mut rng = thread_rng();
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }
        // Palce bomb neighbors
        for y in 0..self.height {
            for x in 0..self.width {
                let coor = Coordinates { x, y };
                if self.is_bomb_at(coor) {
                    continue;
                }
                let num = self.bomb_count_at(coor);
                if num == 0 {
                    continue;
                }
                // Modify coordinates value
                let tile = &mut self[y as usize][x as usize];
                *tile = Tile::BombNeighbor(num);
            }
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs: \n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output())
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }
}

impl Deref for Map {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}