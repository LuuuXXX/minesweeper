use bevy::prelude::Vec2;

#[derive(Clone, Debug)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    // Check whether the position is within the bounds
    pub fn is_bounds(&self, coor: Vec2) -> bool {
        coor.x >= self.position.x
            && coor.y >= self.position.y
            && coor.x <= self.position.x + self.size.x
            && coor.y <= self.position.y + self.size.y
    }
}