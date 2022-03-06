use super::{SpaceObject, SpaceObjectTrait, STAR_COLOURS};
use crate::LehmerRnd;
use cgmath::Point2;

pub type Moon = SpaceObject<_Moon>;

#[derive(Debug, Clone)]
pub struct _Moon {
    pub orbit_distance: f32,
}

impl Moon {
    pub fn new(lehmer: &mut LehmerRnd, Point2 { x, y }: Point2<f32>, orbit: f32) -> Self {
        let orbit_distance = orbit + lehmer.rnd_double(10., 100.);
        let diameter = lehmer.rnd_double(1., 5.);

        Self {
            diameter,
            pos: Point2::new(
                x + orbit_distance * 90.0f32.cos(),
                y + orbit_distance * 90.0f32.sin(),
            ),
            colour: STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize],
            child: Some(_Moon { orbit_distance }),
        }
    }
}

impl SpaceObjectTrait for _Moon {}
