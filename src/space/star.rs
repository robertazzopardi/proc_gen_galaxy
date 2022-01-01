use std::iter;

use cgmath::Point2;

use crate::LehmerRnd;

use super::{planet::Planet, SpaceObject, SpaceObjectTrait, STAR_COLOURS};

#[derive(Debug, Clone)]
pub struct Star {
    pub planets: Vec<SpaceObject<Planet>>,
}

impl SpaceObject<Star> {
    pub fn gen_star(
        sx: i64,
        sy: i64,
        pos: Point2<f32>,
        lehmer: &mut LehmerRnd,
        gen_full_system: bool,
    ) -> Option<Self> {
        lehmer.counter = (sx & 0xFFFF).wrapping_shl(16) | (sy & 0xFFFF);

        let exists = lehmer.rnd_int(0, 20) == 1;
        if !exists {
            return None;
        }

        let diameter = lehmer.rnd_double(10., 40.);

        let colour = STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize];

        Some(SpaceObject {
            diameter,
            pos: pos.clone(),
            colour,
            child: if gen_full_system {
                Some(Star {
                    planets: Self::gen_planets(lehmer, pos.x, pos.y),
                })
            } else {
                None
            },
        })
    }

    fn gen_planets(lehmer: &mut LehmerRnd, x: f32, y: f32) -> Vec<SpaceObject<Planet>> {
        let n_planets = lehmer.rnd_int(0, 10);

        if n_planets == 0 {
            return Vec::new();
        }

        let distance_from_star = lehmer.rnd_double(60., 200.);

        iter::repeat_with(|| SpaceObject::gen_planet(lehmer, x, y, distance_from_star))
            .take(n_planets as usize)
            .collect()
    }
}

impl SpaceObjectTrait for Star {}
