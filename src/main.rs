use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use mine::BoardPlugin;
use mine::resources::board_options::BoardOptions;

fn main() {
    // Init the world
    let mut app = App::new();
    // Window setup
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    
    // Initialize the board with options
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        boom_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    });
    
    // Init the board
    app.add_plugins(BoardPlugin);

    // Init the camera
    app.add_systems(Startup, setup);
    
    // Run game
    app.run();
}

// Set Camera
pub fn setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}