use super::{moon::Moon, SpaceObject, SpaceObjectTrait, STAR_COLOURS};
use crate::LehmerRnd;
use cgmath::Point2;
use std::iter;

#[derive(Debug, Clone)]
pub struct Planet {
    pub moons: Vec<Moon>,
}

impl Planet {
    fn gen_moons(lehmer: &mut LehmerRnd, pos: Point2<f32>) -> Vec<SpaceObject> {
        let n_moons = lehmer.rnd_int(0, 5);

        if n_moons == 0 {
            return Vec::new();
        }

        let orbit = lehmer.rnd_double(6., 20.);

        iter::repeat_with(|| Moon::create(lehmer, pos, orbit))
            .take(n_moons as usize)
            .collect()
    }
}

impl SpaceObjectTrait for Planet {
    fn create(lehmer: &mut LehmerRnd, pos: Point2<f32>, orbit_radius: f32) -> SpaceObject {
        let diameter = lehmer.rnd_double(5., 30.);

        SpaceObject {
            diameter,
            pos: Point2::new(
                pos.x + orbit_radius * 90.0f32.cos(),
                pos.y + orbit_radius * 90.0f32.sin(),
            ),
            colour: STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize],
            orbit_radius,
            satellites: Self::gen_moons(lehmer, pos),
        }
    }
}
