use super::{star::Star, SpaceObject, SpaceObjectTrait};
use crate::{LehmerRnd, X_SECTORS, Y_SECTORS};
use cgmath::Point2;

#[derive(Default)]
pub struct Galaxy {
    pub stars: Vec<SpaceObject>,
}

impl Galaxy {
    pub fn update(&mut self, state_pos: Point2<f32>, lehmer: &mut LehmerRnd) {
        self.stars.clear();

        for x in 0..X_SECTORS {
            for y in 0..Y_SECTORS {
                let sx = (x as f32 + state_pos.x) as i64;
                let sy = (y as f32 + state_pos.y) as i64;

                lehmer.counter = (sx & 0xFFFF).wrapping_shl(16) | (sy & 0xFFFF);

                let star = Star::create(lehmer, Point2::new(x as f32, y as f32), 0.);

                let exists = lehmer.rnd_int(0, 20) == 1;
                if exists {
                    self.stars.push(star);
                }
            }
        }
    }
}
