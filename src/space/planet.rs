use std::iter;

use super::{moon::Moon, SpaceObject, SpaceObjectTrait, STAR_COLOURS};
use crate::LehmerRnd;
use cgmath::Point2;

pub type Planet = SpaceObject<_Planet>;

#[derive(Debug, Clone)]
pub struct _Planet {
    pub orbit_distance: f32,
    pub moons: Vec<Moon>,
}

impl Planet {
    pub fn new(lehmer: &mut LehmerRnd, pos: Point2<f32>, distance_from_star: &mut f32) -> Self {
        let orbit_distance = *distance_from_star;
        *distance_from_star += lehmer.rnd_double(20., 200.);

        let diameter = lehmer.rnd_double(5., 30.);

        Self {
            diameter,
            pos: Point2::new(
                pos.x + orbit_distance * 90.0f32.cos(),
                pos.y + orbit_distance * 90.0f32.sin(),
            ),
            colour: STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize],
            child: Some(_Planet {
                orbit_distance,
                moons: Self::gen_moons(lehmer, pos),
            }),
        }
    }

    fn gen_moons(lehmer: &mut LehmerRnd, pos: Point2<f32>) -> Vec<Moon> {
        let n_moons = lehmer.rnd_int(0, 5);

        if n_moons == 0 {
            return Vec::new();
        }

        let orbit = lehmer.rnd_double(6., 20.);

        iter::repeat_with(|| Moon::new(lehmer, pos, orbit))
            .take(n_moons as usize)
            .collect()
    }
}

impl SpaceObjectTrait for _Planet {}
