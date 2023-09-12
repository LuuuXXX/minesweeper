use std::ops::Add;

use bevy::prelude::Component;

// location
#[derive(Clone, Copy, Component, Debug, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u16;
        let y = ((self.y as i16) + y as i16) as u16;
        Self { x, y }
    }
}
