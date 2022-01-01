use procedural_gen::{State, HEIGHT, WIDTH};
use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseButton, render::Canvas, video::Window, EventPump,
};

pub fn init_sdl() -> Result<(Canvas<Window>, EventPump), String> {
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

pub fn handle_events(event_pump: &mut EventPump, running: &mut bool, state: &mut State) {
    event_pump.poll_iter().for_each(|event| match event {
        Event::Quit { .. } => *running = false,
        Event::KeyDown { keycode, .. } => match keycode {
            Some(Keycode::W) => {
                state.directions[0] = true;
            }
            Some(Keycode::S) => {
                state.directions[1] = true;
            }
            Some(Keycode::A) => {
                state.directions[2] = true;
            }
            Some(Keycode::D) => {
                state.directions[3] = true;
            }
            _ => {}
        },
        Event::KeyUp { keycode, .. } => match keycode {
            Some(Keycode::W) => {
                state.directions[0] = false;
            }
            Some(Keycode::S) => {
                state.directions[1] = false;
            }
            Some(Keycode::A) => {
                state.directions[2] = false;
            }
            Some(Keycode::D) => {
                state.directions[3] = false;
            }
            Some(Keycode::Escape) => *running = false,
            _ => {}
        },
        Event::MouseButtonDown {
            mouse_btn: MouseButton::Left,
            ..
        } => {
            state.lmb_clicked = true;
        }
        Event::MouseButtonUp {
            mouse_btn: MouseButton::Left,
            ..
        } => {
            state.lmb_clicked = false;
        }
        Event::MouseMotion { x, y, .. } => {
            state.mouse_xy.x = x as f32 / 16.;
            state.mouse_xy.y = y as f32 / 16.;
        }
        _ => {}
    });
}
