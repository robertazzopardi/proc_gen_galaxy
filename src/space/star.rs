use crate::{Vec2, LehmerRnd};

use super::{SpaceObject, planet::Planet, STAR_COLOURS, SpaceObjectTrait};

#[derive(Debug, Clone)]
pub struct Star {
    planets: Vec<SpaceObject<Planet>>,
}

impl SpaceObject<Star> {
    pub fn gen_star(
        sx: i64,
        sy: i64,
        pos: Vec2<f32>,
        lehmer: &mut LehmerRnd,
        gen_full_system: bool,
    ) -> Option<Self> {
        lehmer.counter = (sx & 0xFFFF).wrapping_shl(16) | (sy & 0xFFFF);

        let exists = LehmerRnd::rnd_int(lehmer, 0, 20) == 1;
        if !exists {
            return None;
        }

        let diameter = LehmerRnd::rnd_double(lehmer, 10., 40.);

        let colour =
            STAR_COLOURS[LehmerRnd::rnd_int(lehmer, 0, STAR_COLOURS.len() as u32) as usize];

        if !gen_full_system {
            return Some(SpaceObject {
                diameter,
                pos,
                colour,
                child: None,
            });
        }

        Some(SpaceObject {
            diameter,
            pos: pos.clone(),
            colour,
            child: Some(Star {
                planets: Self::gen_planets(lehmer, pos.x, pos.y),
            }),
        })
    }

    fn gen_planets(lehmer: &mut LehmerRnd, x: f32, y: f32) -> Vec<SpaceObject<Planet>> {
        let distance_from_star = LehmerRnd::rnd_double(lehmer, 60., 200.);
        let n_planets = LehmerRnd::rnd_int(lehmer, 0, 10);

        if n_planets == 0 {
            return Vec::new();
        }

        let mut planets = Vec::new();

        for _ in 0..n_planets {
            let orbit_distance = distance_from_star + LehmerRnd::rnd_double(lehmer, 20., 200.);
            let diameter = LehmerRnd::rnd_double(lehmer, 4., 20.);

            planets.push(SpaceObject {
                diameter,
                pos: Vec2 {
                    x: x + orbit_distance * 90.0f32.cos(),
                    y: y + orbit_distance * 90.0f32.sin(),
                },
                colour: STAR_COLOURS
                    [LehmerRnd::rnd_int(lehmer, 0, STAR_COLOURS.len() as u32) as usize],
                child: Some(Planet {
                    orbit_distance,
                    moons: Vec::new(),
                }),
            });
        }

        planets
    }
}

impl SpaceObjectTrait for Star {}
