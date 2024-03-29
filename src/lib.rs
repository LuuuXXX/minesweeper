pub mod components;
pub mod resources;
pub mod systems;
pub mod bounds;

use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashMap;

use resources::board_asset::*;
use resources::board_options::*;
use resources::map::*;
use resources::tile::Tile;
use resources::tile::Tile::*;

use crate::bounds::Bounds2;
use crate::components::coordinates::Coordinates;
use crate::components::uncover::Uncover;
use crate::resources::board::Board;
use crate::systems::event::TileTriggerEvent;
use crate::systems::input::input_handler;
use crate::systems::uncover::trigger_event_handler;
use crate::systems::uncover::uncover_tiles;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, Self::create_board) // 初始化主游戏界面
            .add_systems(Update, input_handler) // 增加输入处理
            .add_systems(Update, trigger_event_handler) // 怎么输出事件绑定对应的处理方式
            .add_systems(Update, uncover_tiles) // 取消覆盖
            .add_event::<TileTriggerEvent>();

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        board_assert: Option<Res<BoardAsset>>,
        window: Query<&Window>
    ) {
        // 拿到初始化borad的参数
        let board_options = match board_options {
            None => panic!("failed to find board options"),
            Some(o) => o.clone(),
        };
        // 拿到资源初始化信息
        let board_assert = match board_assert {
            None => panic!("failed to find board assert"),
            Some(o) => o.clone(),
        };
        // Initialize the map
        let mut map = Map::empty(
            board_options.map_size.0, 
            board_options.map_size.1,
        );
        // Set board bombs
        map.set_bombs(board_options.boom_count);

        #[cfg(feature = "debug")]
        log::info!("{}", map.console_output());

        // We define the size of the tiles in world space
        let tile_size = match board_options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptative { min, max } => Self::adaptative_tile_size(
                window, 
                (min, max), 
                (map.width(), map.height())
            ),
        };
        let board_size = Vec2::new(
            map.width() as f32 * tile_size,
            map.height() as f32 * tile_size,
        );
        // We define the board anchor position 
        let board_position = match board_options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(
                    -(board_size.x / 2.), 
                    -(board_size.y / 2.),  
                    0.,
                ) + offset
            },
            BoardPosition::Custom(p) => p,
        };
        // Init assert
        let mut covered_tiles = HashMap::with_capacity((map.width() * map.height()).into());
        let mut safe_start = None;

        let board_entity = commands
            .spawn(SpriteBundle::default())
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: board_assert.board_material.color,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        texture: board_assert.board_material.texture.clone(),
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
                    board_options.tile_padding,
                    &board_assert,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();
        
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
            covered_tiles,
            entity: board_entity
        });

        // Safe Start, Select a tile to uncover which is empty
        if board_options.safe_place {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }
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
        board_assert: &BoardAsset,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
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
                            color: board_assert.tile_material.color,
                            custom_size: Some(Vec2::splat(
                                size - padding as f32,
                            )),
                            ..Default::default()
                        },
                        texture: board_assert.tile_material.texture.clone(),
                        transform: Transform::from_xyz(
                            ( x as f32 * size ) + ( size / 2. ), 
                            ( y as f32 * size ) + ( size / 2. ), 
                            1.,
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new(format!("Tiles ({}, {})", x, y)))
                    .with_children(|parent| {// Set the split Cover
                    let entity = parent.spawn(
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: board_assert.covered_tile_material.color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., 2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                    // Safe Start
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
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
                                texture: board_assert.bomb_material.texture.clone(),
                                ..Default::default()
                            });
                        });
                    },
                    BombNeighbor(v) => {
                        cmd.insert(components::bomb_neighber::BombNeighbor {count: *v});
                        cmd.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *v, 
                                board_assert, 
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
    fn bomb_count_text_bundle(count: u8, board_assert: &BoardAsset, size: f32) -> Text2dBundle {
        let color = board_assert.bomb_counter_color(count);
        // General a text bundle
        Text2dBundle {
            text: Text { 
                sections: vec![TextSection {
                    value: count.to_string(),
                    style: TextStyle { 
                        font: board_assert.bomb_counter_font.clone(),
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