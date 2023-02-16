use mine::layout::spawn_layout;
use mine::mouse::print_moust_button_input_system;

use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(spawn_layout)
        .add_system(print_moust_button_input_system)
        .run()
}
