// Include the optional argument for game board

use bevy::prelude::{Vec3, Resource};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
    /// Windows adaptative tile size
    Adaptative { min: f32, max: f32 },
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptative {
            min: 10.0,
            max: 50.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    /// Center of the board
    Centered { offset: Vec3 },
    /// Custom
    Custom(Vec3),
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

// Impl Resource trait which is needed for app.insert_resource
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BoardOptions {
    /// Tile map size
    pub map_size: (u16, u16),
    /// bombs count
    pub boom_count: u16,
    /// Board position
    pub position: BoardPosition,
    /// Tile world size
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
    /// Does the board generate a safe place to start
    pub safe_place: bool,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self { 
            map_size: (15, 15), 
            boom_count: 30, 
            position: Default::default(), 
            tile_size: Default::default(), 
            tile_padding: 0., 
            safe_place: false 
        }
    }
}