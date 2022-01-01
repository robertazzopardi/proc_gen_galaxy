extern crate cgmath;
extern crate procedural_gen;
extern crate sdl2;

mod time;
mod window;

use crate::time::Time;
use procedural_gen::State;
use window::{handle_events, init_sdl};

pub fn main() -> Result<(), String> {
    let (mut canvas, mut event_pump) = init_sdl()?;

    let mut running = true;

    let mut time = Time::new();

    let mut state = State::default();

    while running {
        time.diff();

        handle_events(&mut event_pump, &mut running, &mut state);

        time.update(&mut state);

        canvas
            .window_mut()
            .set_title(
                format!(
                    "x: {} y: {} mx: {} my: {}",
                    state.pos.x, state.pos.y, state.mouse_xy.x, state.mouse_xy.y
                )
                .as_str(),
            )
            .unwrap();

        state.render(&mut canvas);
    }

    Ok(())
}
