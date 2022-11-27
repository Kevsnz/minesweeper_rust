extern crate sdl2;

use std::time::{Duration, Instant};

mod draw;
mod field;
use draw::Drawer;
use field::Game;

const FPS: f64 = 60.0;

fn main() {
    let (mut w, mut h, mut mc) = (8, 8, 10);
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 4 {
        w = args[1]
            .parse()
            .expect("Cannot parse first argument (field width)!");
        h = args[2]
            .parse()
            .expect("Cannot parse first argument (field height)!");
        mc = args[3]
            .parse()
            .expect("Cannot parse first argument (mine count)!");

        if w < 8 || h < 8 || w > 200 || h > 200 || mc < 10 {
            println!(
                "Invalid parameters: width: {}, height: {}, mine count: {}!",
                w, h, mc
            );
            println!("200 >= w,h >= 8, mine count >= 10");
            std::process::exit(1);
        }
    }
    let game = field::Game::new(w, h, mc);
    let drawer = Drawer::initialize_game(game.width() as u32, game.height() as u32);
    run_game_loop(drawer, game);
}

fn run_game_loop(mut drawer: Drawer, mut game: Game) {
    let fps_time = Duration::from_secs_f64(1.0 / FPS);
    let mut i = 0.0;
    let mut next = Instant::now().checked_add(fps_time).unwrap();

    loop {
        if drawer.handle_events(&mut game) {
            break;
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
