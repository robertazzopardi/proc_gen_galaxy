use super::{planet::Planet, SpaceObject, SpaceObjectTrait, STAR_COLOURS};
use crate::{LehmerRnd, Point2};
use std::iter;

#[derive(Debug, Clone)]
pub struct Star {
    pub planets: Vec<Planet>,
}

impl Star {
    fn gen_planets(lehmer: &mut LehmerRnd, pos: Point2) -> Vec<SpaceObject> {
        let n_planets = lehmer.rnd_int(0, 10);

        if n_planets == 0 {
            return Vec::new();
        }

        let mut orbit_radius = lehmer.rnd_double(60., 200.);

        iter::repeat_with(|| {
            orbit_radius += lehmer.rnd_double(20., 200.);
            Planet::create(lehmer, pos, orbit_radius)
        })
        .take(n_planets as usize)
        .collect()
    }
}

impl SpaceObjectTrait for Star {
    fn create(lehmer: &mut LehmerRnd, pos: Point2, orbit_radius: f32) -> SpaceObject {
        let diameter = lehmer.rnd_double(10., 40.);

        let colour = STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize];

        let gen_full_system = lehmer.rnd_int(0, 10) == 1;

        SpaceObject {
            diameter,
            pos,
            colour,
            orbit_radius,
            satellites: if gen_full_system {
                Star::gen_planets(lehmer, pos)
            } else {
                Vec::new()
            },
        }
    }
}
