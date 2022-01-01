use cgmath::Point2;
use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use space::{galaxy::Galaxy, star::Star, SpaceObject};

mod space;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const X_SECTORS: u32 = WIDTH / 16;
pub const Y_SECTORS: u32 = HEIGHT / 16;

#[derive(Debug, Default)]
pub struct LehmerRnd {
    pub counter: i64,
}

impl LehmerRnd {
    fn next(&mut self) -> u32 {
        self.counter += 0xe120fc15;
        let mut tmp: i64;
        tmp = self.counter.wrapping_mul(0x4a39b70d);
        let m1 = (tmp >> 32) ^ tmp;
        tmp = m1.wrapping_mul(0x12fad5c9);
        let m2 = (tmp >> 32) ^ tmp;
        m2 as u32
    }

    pub fn rnd_int(&mut self, min: u32, max: u32) -> u32 {
        (self.next() % (max - min)) + min
    }

    pub fn rnd_double(&mut self, min: f32, max: f32) -> f32 {
        (self.next() as f32 / 0x7FFFFFFF as f32) * (max - min) + min
    }
}

pub struct State {
    pub pos: Point2<f32>,
    pub directions: [bool; 4],
    pub mouse_xy: Point2<f32>,
    pub lmb_clicked: bool,
    pub galaxy: Galaxy,
    pub lehmer: LehmerRnd,
    pub selected_system: Option<SpaceObject<Star>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: Point2::new(0., 0.),
            directions: [false, false, false, false],
            mouse_xy: Point2::new(0., 0.),
            lmb_clicked: false,
            galaxy: Galaxy::default(),
            lehmer: LehmerRnd { counter: 0 },
            selected_system: None,
        }
    }
}

impl State {
    pub fn update(&mut self, dt: f32) {
        if self.directions[0] {
            self.pos.y -= 50. * dt;
        }
        if self.directions[1] {
            self.pos.y += 50. * dt;
        }
        if self.directions[2] {
            self.pos.x -= 50. * dt;
        }
        if self.directions[3] {
            self.pos.x += 50. * dt;
        }

        self.galaxy.update(self.pos, &mut self.lehmer);

        if self.lmb_clicked {
            let sx = (self.mouse_xy.x.floor() as f32 + self.pos.x) as i64;
            let sy = (self.mouse_xy.y.floor() as f32 + self.pos.y) as i64;

            let star_system = SpaceObject::gen_star(
                Point2::new(sx, sy),
                Point2::new(self.pos.x.floor() as f32, self.pos.y.floor() as f32),
                &mut self.lehmer,
                true,
            );

            self.selected_system = star_system.or(None);
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        self.galaxy.stars.iter().for_each(|system| {
            let _ = canvas.filled_circle(
                system.pos.x as i16 * 16 + 8,
                system.pos.y as i16 * 16 + 8,
                (system.diameter / 8.) as i16,
                system.colour,
            );

            if self.mouse_xy.x.floor() == system.pos.x.floor()
                && self.mouse_xy.y.floor() == system.pos.y.floor()
            {
                let _ = canvas.aa_circle(
                    system.pos.x as i16 * 16 + 8,
                    system.pos.y as i16 * 16 + 8,
                    12,
                    Color::WHITE,
                );
            }
        });

        if let Some(system) = &self.selected_system {
            // Container
            canvas.set_draw_color(Color::BLUE);
            let _ = canvas.fill_rect(Rect::new(
                10,
                (HEIGHT / 2u32) as i32,
                WIDTH - 20,
                (HEIGHT / 2) - 10,
            ));
            canvas.set_draw_color(Color::WHITE);
            let _ = canvas.draw_rect(Rect::new(
                10,
                (HEIGHT / 2u32) as i32,
                WIDTH - 20,
                (HEIGHT / 2) - 10,
            ));

            // Star
            let x = (WIDTH / 16u32) as i16;
            let y = ((HEIGHT / 2u32) as f32) as i16 + (HEIGHT / 4u32) as i16;

            let _ = canvas.filled_circle(x, y, (system.diameter / 2.) as i16, system.colour);

            // Planets
            if let Some(star) = &system.child {
                star.planets.iter().for_each(|planet| {
                    let planet_x = x + (planet.child.as_ref().unwrap().orbit_distance * 0.5) as i16;
                    let _ = canvas.filled_circle(
                        planet_x,
                        y,
                        (planet.diameter / 2.) as i16,
                        planet.colour,
                    );

                    // Moons
                    if let Some(planet) = &planet.child {
                        planet.moons.iter().for_each(|moon| {
                            let _ = canvas.filled_circle(
                                planet_x,
                                y + (moon.child.as_ref().unwrap().orbit_distance * 0.8) as i16,
                                (moon.diameter / 2.0) as i16,
                                moon.colour,
                            );
                        });
                    }
                });
            }
        }

        canvas.present();
    }
}
