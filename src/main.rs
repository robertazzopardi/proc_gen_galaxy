mod time;
mod window;

use crate::time::Time;
use procedural_gen::State;
use window::{init_sdl, main_loop};

pub fn main() -> Result<(), String> {
    let (canvas, event_pump) = init_sdl()?;

    let running = true;

    let time = Time::new();

    let state = State::default();

    main_loop(running, time, event_pump, state, canvas);

    Ok(())
}
