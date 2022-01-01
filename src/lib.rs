use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use space::{galaxy::Galaxy, star::Star, SpaceObject};

mod space;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const X_SECTORS: u32 = WIDTH / 16;
pub const Y_SECTORS: u32 = HEIGHT / 16;

#[derive(Debug, Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

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
    pub pos: Vec2<f32>,
    pub directions: [bool; 4],
    pub mouse_xy: Vec2<f32>,
    pub lmb_clicked: bool,
    pub galaxy: Galaxy,
    pub lehmer: LehmerRnd,
    pub selected_system: Option<SpaceObject<Star>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Vec2::new(0., 0.),
            directions: [false, false, false, false],
            mouse_xy: Vec2::new(0., 0.),
            lmb_clicked: false,
            galaxy: Galaxy::new(),
            lehmer: LehmerRnd { counter: 0 },
            selected_system: None,
        }
    }

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

        self.galaxy.update(self.pos.clone(), &mut self.lehmer);

        if self.lmb_clicked {
            let sx = (self.mouse_xy.x as f32 + self.pos.x) as i64;
            let sy = (self.mouse_xy.y as f32 + self.pos.y) as i64;

            let star_system = SpaceObject::gen_star(
                sx,
                sy,
                Vec2::new(self.mouse_xy.x as f32, self.mouse_xy.y as f32),
                &mut self.lehmer,
                true,
            );

            if star_system.is_some() {
                self.selected_system = star_system
            } else {
                self.selected_system = None
            }
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
            println!("{:?}", system);
        }

        canvas.present();
    }
}
