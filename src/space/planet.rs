use std::iter;

use cgmath::Point2;

use crate::LehmerRnd;

use super::{moon::Moon, SpaceObject, SpaceObjectTrait, STAR_COLOURS};

#[derive(Debug, Clone)]
pub struct Planet {
    pub orbit_distance: f32,
    pub moons: Vec<SpaceObject<Moon>>,
}

impl SpaceObject<Planet> {
    pub fn gen_planet(lehmer: &mut LehmerRnd, x: f32, y: f32, distance_from_star: f32) -> Self {
        let orbit_distance = distance_from_star + lehmer.rnd_double(20., 200.);
        let diameter = lehmer.rnd_double(4., 20.);

        Self {
            diameter,
            pos: Point2::new(
                x + orbit_distance * 90.0f32.cos(),
                y + orbit_distance * 90.0f32.sin(),
            ),
            colour: STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize],
            child: Some(Planet {
                orbit_distance,
                moons: Self::gen_moons(lehmer, x, y),
            }),
        }
    }

    fn gen_moons(lehmer: &mut LehmerRnd, x: f32, y: f32) -> Vec<SpaceObject<Moon>> {
        let n_moons = lehmer.rnd_int(0, 5);

        if n_moons == 0 {
            return Vec::new();
        }

        let distance_from_planet = lehmer.rnd_double(6., 20.);

        iter::repeat_with(|| SpaceObject::gen_moon(lehmer, x, y, distance_from_planet))
            .take(n_moons as usize)
            .collect()
    }
}

impl SpaceObjectTrait for Planet {}
