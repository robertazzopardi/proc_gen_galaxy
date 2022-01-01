pub mod galaxy;
pub mod moon;
pub mod planet;
pub mod star;

use cgmath::Point2;
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

pub trait SpaceObjectTrait {}

#[derive(Debug, Clone)]
pub struct SpaceObject<T: SpaceObjectTrait> {
    pub diameter: f32,
    pub pos: Point2<f32>,
    pub colour: Color,
    pub child: Option<T>,
}
