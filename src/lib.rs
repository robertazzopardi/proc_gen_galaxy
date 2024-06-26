use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use space::{galaxy::Galaxy, SpaceObject};

mod space;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const X_SECTORS: u32 = WIDTH / 16;
pub const Y_SECTORS: u32 = HEIGHT / 16;

#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default)]
pub struct LehmerRnd {
    pub counter: i64,
}

impl LehmerRnd {
    fn next(&mut self) -> u32 {
        self.counter += 0xe120_fc15;
        let mut tmp = self.counter.wrapping_mul(0x4a39_b70d);
        let m1 = (tmp >> 32) ^ tmp;
        tmp = m1.wrapping_mul(0x12fa_d5c9);
        let m2 = (tmp >> 32) ^ tmp;
        m2 as u32
    }

    pub fn rnd_int(&mut self, min: u32, max: u32) -> u32 {
        (self.next() % (max - min)) + min
    }

    pub fn rnd_double(&mut self, min: f32, max: f32) -> f32 {
        (self.next() as f32 / 0x7FFF_FFFF as f32) * (max - min) + min
    }
}

pub struct State {
    pub pos: Point2,
    pub directions: [bool; 4],
    pub mouse_xy: Point2,
    pub lmb_clicked: bool,
    pub galaxy: Galaxy,
    pub lehmer: LehmerRnd,
    pub selected_star: Option<SpaceObject>,
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
            selected_star: None,
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
            let exists = self.galaxy.stars.iter().find(|star| {
                star.pos.x.floor() == self.mouse_xy.x.floor()
                    && star.pos.y.floor() == self.mouse_xy.y.floor()
            });

            self.selected_star = exists.cloned();
        }
    }

    fn render_stars(&mut self, canvas: &Canvas<Window>) {
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
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        self.render_stars(canvas);

        self.render_selected_star(canvas);

        canvas.present();
    }

    fn render_selected_star(&mut self, canvas: &mut Canvas<Window>) {
        if let Some(star) = &self.selected_star {
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
            let x = i16::try_from(WIDTH / 16u32).unwrap();
            let y = ((HEIGHT / 2u32) as f32) as i16 + i16::try_from(HEIGHT / 4u32).unwrap();

            let _ = canvas.filled_circle(x, y, (star.diameter / 2.) as i16, star.colour);

            // Planets
            star.satellites.iter().for_each(|planet| {
                let planet_x = x + (planet.orbit_radius * 0.5) as i16;
                let _ =
                    canvas.filled_circle(planet_x, y, (planet.diameter / 2.) as i16, planet.colour);

                // Moons
                planet.satellites.iter().for_each(|moon| {
                    let _ = canvas.filled_circle(
                        planet_x,
                        y + (moon.orbit_radius * 0.8) as i16,
                        (moon.diameter / 2.0) as i16,
                        moon.colour,
                    );
                });
            });
        }
    }
}
