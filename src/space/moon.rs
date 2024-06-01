use super::{SpaceObject, SpaceObjectTrait, STAR_COLOURS};
use crate::LehmerRnd;
use cgmath::Point2;

#[derive(Debug, Clone)]
pub struct Moon;

impl SpaceObjectTrait for Moon {
    fn create(lehmer: &mut LehmerRnd, point: Point2<f32>, orbit: f32) -> SpaceObject {
        let orbit_radius = orbit + lehmer.rnd_double(10., 100.);
        let diameter = lehmer.rnd_double(1., 5.);

        SpaceObject {
            diameter,
            pos: Point2::new(
                point.x + orbit_radius * 90.0f32.cos(),
                point.y + orbit_radius * 90.0f32.sin(),
            ),
            colour: STAR_COLOURS[lehmer.rnd_int(0, STAR_COLOURS.len() as u32) as usize],
            orbit_radius,
            satellites: Vec::new(),
        }
    }
}
