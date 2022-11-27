extern crate sdl2;

use std::time::{Duration, Instant};

mod draw;
mod field;
use draw::Drawer;
use field::Game;

const FPS: f64 = 60.0;

fn main() {
    let game = field::Game::new(10, 10, 16);
    let drawer = Drawer::initialize_game(game.width() as u32, game.height() as u32);
    run_game_loop(drawer, game);
}

fn run_game_loop(mut drawer: Drawer, mut game: Game) {
    let fps_time = Duration::from_secs_f64(1.0 / FPS);
    let mut i = 0.0;
    let mut next = Instant::now().checked_add(fps_time).unwrap();

    'running: loop {
        if drawer.handle_events(&mut game) {
            break 'running;
        }

        i = (i + 0.03) % (std::f64::consts::PI * 2.0);
        drawer.draw_screen(i, &game);

        let now = Instant::now();
        let remaining = next.duration_since(now);
        if !remaining.is_zero() {
            ::std::thread::sleep(remaining);
        }
        next = now.checked_add(fps_time).unwrap();
    }
}
