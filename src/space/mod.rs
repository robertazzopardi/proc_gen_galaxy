pub mod galaxy;
pub mod moon;
pub mod planet;
pub mod star;

use crate::{LehmerRnd, Point2};
use sdl2::pixels::Color;

const STAR_COLOURS: [Color; 7] = [
    Color::RGB(175, 201, 255),
    Color::RGB(199, 216, 255),
    Color::RGB(255, 244, 243),
    Color::RGB(255, 229, 207),
    Color::RGB(255, 217, 178),
    Color::RGB(255, 199, 142),
    Color::RGB(255, 166, 81),
];

#[derive(Debug, Clone)]
pub struct SpaceObject {
    pub diameter: f32,
    pub pos: Point2,
    pub colour: Color,
    pub orbit_radius: f32,
    pub satellites: Vec<SpaceObject>,
}

pub trait SpaceObjectTrait {
    fn create(lehmer: &mut LehmerRnd, point: Point2, orbit_radius: f32) -> SpaceObject;
}
