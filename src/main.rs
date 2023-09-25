use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use mine::BoardPlugin;
use mine::resources::board_asset::{SpriteMaterial, BoardAsset};
use mine::resources::board_options::BoardOptions;

fn main() {
    // Init the world
    let mut app = App::new();
    // Window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin { // 创建默认游戏窗口大小
        primary_window: Some(Window {
            title: "Mine Sweeper".to_string(),
            resolution: (700., 800.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tell wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tell wasm to override default event handler, like F5, CTRL+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }));

    // Init the board
    app
        .add_plugins(BoardPlugin)
        .add_systems(Startup, setup)
        .run();
}

// Set Camera
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
    // Board Option
    commands.insert_resource(BoardOptions { // 初始化游戏资源
        map_size: (20, 20),
        boom_count: 40,
        tile_padding: 3.0,
        safe_place: true,
        ..Default::default()
    });
    // Board Asset
    commands.insert_resource(BoardAsset {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial { 
            color: Color::GRAY, 
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAsset::default_color(),
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png")
        },
        bomb_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/bomb.png")
        },
    });
}