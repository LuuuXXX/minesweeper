use bevy::prelude::*;
use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::log;

use crate::resources::board::Board;

use super::event::TileTriggerEvent;

pub fn input_handler(
    window: Query<&Window>,
    board: Option<Res<Board>>,
    mut button_event: EventReader<MouseButtonInput>,
    mut tile_trigger_event: EventWriter<TileTriggerEvent>
) {
    let window = window.single();

    for event in button_event.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                #[cfg(feature = "debug")]
                log::info!("Mouse button pressed: {:?} at {}", event.button, pos);
                if let Some(board) = &board {
                    let coordinates = board.mouse_position(window, pos);
                    if let Some(coor) = coordinates {
                        match event.button {
                            MouseButton::Left => {
                                log::info!("Trying to uncover tile at: {:?}", coor);
                                tile_trigger_event.send(TileTriggerEvent(coor));
                            },
                            MouseButton::Right => {
                                log::info!("Trying to flag bomb at: {:?}", coor);
                            },
                            MouseButton::Middle => todo!(),
                            MouseButton::Other(_) => todo!(),
                        }
                    }
                }
            }
        }
    }
}