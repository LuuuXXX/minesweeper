use bevy::{
    input::mouse::{MouseButtonInput},
    prelude::*,
};

// this system prints out all mouse events as they come in
pub fn print_moust_button_input_system(mut mouse_button_input_events: EventReader<MouseButtonInput>) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }
}