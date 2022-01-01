use crate::{LehmerRnd, Vec2, X_SECTORS, Y_SECTORS};

use super::{star::Star, SpaceObject};

pub struct Galaxy {
    pub stars: Vec<SpaceObject<Star>>,
}

impl Galaxy {
    pub fn new() -> Self {
        Self { stars: Vec::new() }
    }

    pub fn update(&mut self, state_pos: Vec2<f32>, lehmer: &mut LehmerRnd) {
        self.stars.clear();

        for x in 0..X_SECTORS {
            for y in 0..Y_SECTORS {
                let star_system = SpaceObject::gen_star(
                    (x as f32 + state_pos.x) as i64,
                    (y as f32 + state_pos.y) as i64,
                    Vec2::new(x as f32, y as f32),
                    lehmer,
                    false,
                );

                if let Some(system) = star_system {
                    self.stars.push(system);
                }
            }
        }
    }

    // pub fn render(&self, canvas: &mut Canvas<Window>, mouse_pos: Vec2<f32>, mouse_clicked: bool) {
    //     canvas.set_draw_color(Color::BLACK);
    //     canvas.clear();

    //     self.stars.iter().for_each(|system| {
    //         let _ = canvas.filled_circle(
    //             system.pos.x as i16 * 16 + 8,
    //             system.pos.y as i16 * 16 + 8,
    //             (system.diameter / 8.) as i16,
    //             system.colour,
    //         );

    //         if mouse_pos.x.floor() == system.pos.x.floor()
    //             && mouse_pos.y.floor() == system.pos.y.floor()
    //         {
    //             let _ = canvas.aa_circle(
    //                 system.pos.x as i16 * 16 + 8,
    //                 system.pos.y as i16 * 16 + 8,
    //                 12,
    //                 Color::WHITE,
    //             );
    //         }
    //     });

    //     if mouse_clicked {
    //         let star_system = SpaceObject::gen_star(
    //             (x as f32 + state_pos.x) as i64,
    //             (y as f32 + state_pos.y) as i64,
    //             Vec2::new(x as f32, y as f32),
    //             lehmer,
    //             false,
    //         );
    //     }

    //     canvas.present();
    // }
}
