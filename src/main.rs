extern crate sdl2;

use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, render::Canvas,
    video::Window, EventPump,
};
use std::time::{Duration, Instant};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const X_SECTORS: u32 = WIDTH / 16;
const Y_SECTORS: u32 = HEIGHT / 16;

const STAR_COLOURS: [Color; 7] = [
    Color::RGB(175, 201, 255),
    Color::RGB(199, 216, 255),
    Color::RGB(255, 244, 243),
    Color::RGB(255, 229, 207),
    Color::RGB(255, 217, 178),
    Color::RGB(255, 199, 142),
    Color::RGB(255, 166, 81),
];

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Default)]
struct LehmerRnd {
    counter: i64,
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

    fn rnd_int(lehmer: &mut LehmerRnd, min: u32, max: u32) -> u32 {
        (lehmer.next() % (max - min)) + min
    }

    fn rnd_double(lehmer: &mut LehmerRnd, min: f32, max: f32) -> f32 {
        (lehmer.next() as f32 / 0x7FFFFFFF as f32) * (max - min) + min
    }
}

#[derive(Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Moon {
    diameter: f32,
    pos: Vec2<f32>,
    colour: Color,
}

#[derive(Debug)]
struct Planet {
    diameter: f32,
    pos: Vec2<f32>,
    colour: Color,
    moons: Vec<Moon>,
}

#[derive(Debug)]
struct Star {
    diameter: f32,
    pos: Vec2<f32>,
    colour: Color,
    // planets: Vec<Planet>,
}

impl Star {
    fn new(
        sx: i64,
        sy: i64,
        pos: Vec2<f32>,
        lehmer: &mut LehmerRnd,
        gen_full_system: bool,
    ) -> Option<Self> {
        lehmer.counter = (sx & 0xFFFF).wrapping_shl(16) | (sy & 0xFFFF);

        let exists = LehmerRnd::rnd_int(lehmer, 0, 20) == 1;
        if !exists {
            return None;
        }

        let diameter = LehmerRnd::rnd_double(lehmer, 10., 40.);

        let colour =
            STAR_COLOURS[LehmerRnd::rnd_int(lehmer, 0, STAR_COLOURS.len() as u32) as usize];

        if !gen_full_system {
            return Some(Self {
                diameter,
                pos,
                colour,
            });
        }

        Some(Self {
            diameter,
            pos,
            colour,
        })
    }
}

struct Galaxy {
    stars: Vec<Star>,
    pos: Vec2<f32>,
    direction: Direction,
    lehmer: LehmerRnd,
}

impl Galaxy {
    fn new() -> Self {
        Self {
            stars: Vec::new(),
            pos: Vec2::new(0., 0.),
            direction: Direction::None,
            lehmer: LehmerRnd::default(),
        }
    }

    fn update(&mut self, dt: f32) {
        if self.direction == Direction::Up {
            self.pos.y -= 50. * dt;
        }
        if self.direction == Direction::Down {
            self.pos.y += 50. * dt;
        }
        if self.direction == Direction::Left {
            self.pos.x -= 50. * dt;
        }
        if self.direction == Direction::Right {
            self.pos.x += 50. * dt;
        }

        self.stars.clear();

        for x in 0..X_SECTORS {
            for y in 0..Y_SECTORS {
                let star_system = Star::new(
                    (x as f32 + self.pos.x) as i64,
                    (y as f32 + self.pos.y) as i64,
                    Vec2::new(x as f32, y as f32),
                    &mut self.lehmer,
                    false,
                );

                if let Some(system) = star_system {
                    self.stars.push(system);
                }
            }
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        self.stars.iter().for_each(|system| {
            let _ = canvas.filled_circle(
                system.pos.x as i16 * 16 + 8,
                system.pos.y as i16 * 16 + 8,
                (system.diameter / 8.) as i16,
                system.colour,
            );
        });

        canvas.present();
    }
}

fn init_sdl() -> Result<(Canvas<Window>, EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;
    Ok((canvas, event_pump))
}

fn handle_events(event_pump: &mut EventPump, running: &mut bool, galaxy: &mut Galaxy) {
    event_pump.poll_iter().for_each(|event| match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => *running = false,
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => galaxy.direction = Direction::Up,
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => galaxy.direction = Direction::Down,
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => galaxy.direction = Direction::Left,
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => galaxy.direction = Direction::Right,
        Event::KeyUp {
            keycode: Some(Keycode::W | Keycode::S | Keycode::A | Keycode::D),
            ..
        } => galaxy.direction = Direction::None,
        _ => {}
    });
}

struct Time {
    dt: Duration,
    t: Duration,
    now: Instant,
    acc: Duration,
}

impl Time {
    fn new() -> Self {
        Self {
            dt: Duration::from_secs_f32(0.01666),
            t: Duration::ZERO,
            now: std::time::Instant::now(),
            acc: Duration::ZERO,
        }
    }

    fn diff(&mut self) {
        let new_time = std::time::Instant::now();
        let frame_time = new_time - self.now; // from ns to s
        self.now = new_time;
        self.acc += frame_time;
    }

    fn update(&mut self, galaxy: &mut Galaxy) {
        while self.acc >= self.dt {
            galaxy.update(self.dt.as_secs_f32());
            self.acc -= self.dt;
            self.t += self.dt;
        }
    }
}

pub fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = init_sdl()?;

    let mut galaxy = Galaxy::new();

    let mut running = true;

    let mut time = Time::new();

    while running {
        time.diff();

        handle_events(&mut event_pump, &mut running, &mut galaxy);

        time.update(&mut galaxy);

        canvas
            .window_mut()
            .set_title(format!("x: {} y: {}", galaxy.pos.x, galaxy.pos.y).as_str())
            .unwrap();

        galaxy.render(&mut canvas);
    }

    Ok(())
}
