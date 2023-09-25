use bevy::prelude::*;
use bevy_render::texture::DEFAULT_IMAGE_HANDLE;
// 所有的资源从这里进行控制

#[derive(Debug, Clone)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl Default for SpriteMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            texture: DEFAULT_IMAGE_HANDLE.typed()
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub struct BoardAsset {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub bomb_counter_font: Handle<Font>,
    pub bomb_counter_colors: Vec<Color>,
    pub flag_material: SpriteMaterial,
    pub bomb_material: SpriteMaterial,
}

impl BoardAsset {
    // default
    pub fn default_color() -> Vec<Color> {
        vec![
            Color::WHITE,
            Color::GREEN,
            Color::YELLOW,
            Color::ORANGE,
            Color::PURPLE,
        ]
    }

    // Reture color by matching bomb counter
    pub fn bomb_counter_color(&self, counter: u8) -> Color {
        let counter = counter.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(counter) {
            Some(color) => *color,
            None => Color::WHITE,
        }
    }
}