use super::{SpaceObject, SpaceObjectTrait};

#[derive(Debug, Clone)]
pub struct Moon {
    pub orbit_distance: f32,
}

impl SpaceObject<Moon> {}

impl SpaceObjectTrait for Moon {}
