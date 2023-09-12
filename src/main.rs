use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy::log;

use mine::{BoardPlugin, AppState};
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
        safe_place: true,
        ..Default::default()
    });
    
    // Init the board
    app.add_plugins(BoardPlugin {
        running_state: AppState::InGame,
    });

    // Init the camera
    app.add_systems(Startup, setup);

    // Init State handler
    app.add_state::<AppState>()
        .add_systems(Update, state_handler);
    
    // Run game
    app.run();
}

// Set Camera
pub fn setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::info!("clearing detected");
        if state.get() == &AppState::InGame {
            log::info!("clearing game");
            *state = State::new(AppState::Out);
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::info!("loading detected");
        if state.get() == &AppState::Out {
            log::info!("loading game");
            *state = State::new(AppState::InGame);
        }
    }
}