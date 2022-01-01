use super::{moon::Moon, SpaceObject, SpaceObjectTrait};

#[derive(Debug, Clone)]
pub struct Planet {
    pub orbit_distance: f32,
    pub moons: Vec<Moon>,
}

impl SpaceObject<Planet> {}

impl SpaceObjectTrait for Planet {}
