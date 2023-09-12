use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct BombNeighbor {
    // Number of neighbors bombs
    pub count: u8,
}
