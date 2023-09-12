use bevy::prelude::Event;

use crate::components::coordinates::Coordinates;

#[derive(Debug, Clone, Copy, Event)]
pub struct TileTriggerEvent(pub Coordinates);