mod game;
mod render;

extern crate sdl2;
extern crate gl;

use std::time::Instant;
use crate::game::Game;

const FPS: f64 = 144.;
const CELL_COUNT: usize = 1_000_000;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new()?;


    unsafe {
        game.init();
    }

    const SECONDS_IN_FRAME: f64 = 1. / FPS;
    let mut last_frame = Instant::now();

    while game.is_running() {
        game.handle_events();

        let now = Instant::now();
        let delta = now.duration_since(last_frame).as_secs_f64();

        if delta >= SECONDS_IN_FRAME {
            last_frame = now;

            game.update(delta);
            unsafe { game.render(delta) }
        }
    }

    Ok(())
}
