pub mod components;
pub mod resources;
pub mod systems;
pub mod bounds;

use bevy::log;
use bevy::prelude::*;

use bevy::utils::HashMap;
use resources::map::*;
use resources::board_options::*;
use resources::tile::Tile::*;

use crate::bounds::Bounds2;
use crate::components::coordinates::Coordinates;
use crate::resources::board::Board;
use crate::systems::event::TileTriggerEvent;
use crate::systems::input::input_handler;
use crate::systems::uncover::trigger_event_handler;
use crate::systems::uncover::uncover_tiles;


pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        app.add_systems(Update, input_handler);
        // Logic
        app.add_systems(Update, trigger_event_handler)
            .add_systems(Update, uncover_tiles)
            .add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Query<&Window>,
        asset_server: Res<AssetServer>
    ) {
        // Set board option while creating a new board
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };
        // Initialize the map
        let mut map = Map::empty(
            options.map_size.0, 
            options.map_size.1,
        );
        map.set_bombs(options.boom_count);
        #[cfg(feature = "debug")]
        log::info!("{}", map.console_output());

        // We define the size of the tiles in world space
        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptative { min, max } => Self::adaptative_tile_size(
                window, 
                (min, max), 
                (map.width(), map.height())
            ),
        };

        log::info!("tile_size: {:?}", tile_size);

        let board_size = Vec2::new(
            map.width() as f32 * tile_size,
            map.height() as f32 * tile_size,
        );

        log::info!("board_size: {:?}", board_size);

        // We define the board anchor position 
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(
                    -(board_size.x / 2.), 
                    -(board_size.y / 2.),  
                    0.,
                ) + offset
            },
            BoardPosition::Custom(p) => p,
        };

        log::info!("board_position: {:?}", board_position);

        // Init assert
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");
        let mut covered_tiles = HashMap::with_capacity((map.width() * map.height()).into());

        commands
            .spawn(SpriteBundle::default())
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            board_size.x / 2., 
                            board_size.y / 2., 
                            0.,
                        ),
                        ..Default::default()
                    }).insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &map,
                    tile_size,
                    options.tile_padding,
                    Color::GRAY,
                    bomb_image, 
                    font,
                    Color::DARK_GRAY,
                    &mut covered_tiles
                );
            });
        
        // Add the main board resource
        commands.insert_resource(Board {
            tile_map: map,
            bounds: Bounds2 {
                position: Vec2 {
                    x: board_position.x,
                    y: board_position.y,
                },
                size: board_size,
            },
            tile_size: tile_size,
            covered_tiles
        });
    }

    /// Computes a tile size that matches the window according to the tile map size
    fn adaptative_tile_size(
        window: Query<&Window>,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let window = window.single();
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    /// Spawn tile
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        map: &Map,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>
    ) {
        // Tiles
        for (y, line) in map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: (map.len() -1 - y) as u16,
                };
                let mut cmd = parent.spawn_empty();
                cmd.insert(SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::splat(
                                size - padding as f32,
                            )),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            ( x as f32 * size ) + ( size / 2. ), 
                            ( y as f32 * size ) + ( size / 2. ), 
                            1.,
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new(format!("Tiles ({}, {})", x, y)));

                // Set the split Cover
                cmd.with_children(|parent| {
                    let entity = parent.spawn(
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                });

                // Inset bomb sprites
                match tile {
                    Bomb => {
                        cmd.insert(components::bomb::Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK,
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    },
                    BombNeighbor(v) => {
                        cmd.insert(components::bomb_neighber::BombNeighbor {count: *v});
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v, 
                                font.clone(), 
                                size - padding,
                            ));
                        });
                    },
                    Empty => (),
                }
            }
        }


    }

    /// Generates the bomb counter txtx 2D Bundle for a given value
    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
              1 => Color::WHITE,
              2 => Color::GREEN,
              3 => Color::YELLOW,
              4 => Color::ORANGE,
              _ => Color::PURPLE,
            },
        );
        // General a text bundle
        Text2dBundle {
            text: Text { 
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle { 
                        font, 
                        font_size: size, 
                        color 
                    },
                }],
                alignment: TextAlignment::Center,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }
}