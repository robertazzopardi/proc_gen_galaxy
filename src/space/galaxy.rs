use cgmath::Point2;

use crate::{LehmerRnd, X_SECTORS, Y_SECTORS};

use super::{star::Star, SpaceObject};

#[derive(Default)]
pub struct Galaxy {
    pub stars: Vec<SpaceObject<Star>>,
}

impl Galaxy {
    pub fn update(&mut self, state_pos: Point2<f32>, lehmer: &mut LehmerRnd) {
        self.stars.clear();

        for x in 0..X_SECTORS {
            for y in 0..Y_SECTORS {
                let sx = (x as f32 + state_pos.x) as i64;
                let sy = (y as f32 + state_pos.y) as i64;

                let star_system = SpaceObject::gen_star(
                    Point2::new(sx, sy),
                    Point2::new(x as f32, y as f32),
                    lehmer,
                    false,
                );

                if let Some(system) = star_system {
                    self.stars.push(system);
                }
            }
        }
    }
}
