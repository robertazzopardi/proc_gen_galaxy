use super::{planet::Planet, SpaceObject, SpaceObjectTrait, STAR_COLOURS};
use crate::LehmerRnd;
use cgmath::Point2;
use std::iter;

pub type Star = SpaceObject<_Star>;

#[derive(Debug, Clone)]
pub struct _Star {
    pub planets: Vec<Planet>,
}

impl Star {
    pub fn new(
        s_xy: Point2<i64>,
        pos: Point2<f32>,
        lehmer: &mut LehmerRnd,
        gen_full_system: bool,
    ) -> Option<Self> {
        lehmer.counter = (s_xy.x & 0xFFFF).wrapping_shl(16) | (s_xy.y & 0xFFFF);

        let exists = lehmer.rnd_int(0, 20) == 1;
        if !exists {
            return None;
        }

        let diameter = lehmer.rnd_double(10., 40.);

        let colour = STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize];

        Some(SpaceObject {
            diameter,
            pos,
            colour,
            child: if gen_full_system {
                Some(_Star {
                    planets: Self::gen_planets(lehmer, pos),
                })
            } else {
                None
            },
        })
    }

    fn gen_planets(lehmer: &mut LehmerRnd, pos: Point2<f32>) -> Vec<Planet> {
        let n_planets = lehmer.rnd_int(0, 10);

        if n_planets == 0 {
            return Vec::new();
        }

        let mut orbit = lehmer.rnd_double(60., 200.);

        iter::repeat_with(|| Planet::new(lehmer, pos, &mut orbit))
            .take(n_planets as usize)
            .collect()
    }
}

impl SpaceObjectTrait for _Star {}
