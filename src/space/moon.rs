use cgmath::Point2;

use crate::LehmerRnd;

use super::{SpaceObject, SpaceObjectTrait, STAR_COLOURS};

#[derive(Debug, Clone)]
pub struct Moon {
    pub orbit_distance: f32,
}

impl SpaceObject<Moon> {
    pub fn gen_moon(lehmer: &mut LehmerRnd, x: f32, y: f32, distance_from_planet: f32) -> Self {
        let orbit_distance = distance_from_planet + lehmer.rnd_double(20., 200.);
        let diameter = lehmer.rnd_double(1., 5.);

        Self {
            diameter,
            pos: Point2::new(
                x + orbit_distance * 90.0f32.cos(),
                y + orbit_distance * 90.0f32.sin(),
            ),
            colour: STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize],
            child: Some(Moon { orbit_distance }),
        }
    }
}

impl SpaceObjectTrait for Moon {}
