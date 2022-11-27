use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use crate::field::{GameState, TileContent};
use crate::Game;

pub struct Drawer<'a> {
    // sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    assets: Assets<'a>,
}

pub struct Assets<'a> {
    pub borders: Surface<'a>,
    pub numbers: Surface<'a>,
    pub faces: Surface<'a>,
    pub tiles: Surface<'a>,
}

impl<'a> Assets<'a> {
    pub fn load() -> Assets<'a> {
        let borders = Surface::load_bmp("assets\\borders.bmp").expect("Cannot load surface!");
        let numbers = Surface::load_bmp("assets\\numbers.bmp").expect("Cannot load surface!");
        let faces = Surface::load_bmp("assets\\faces.bmp").expect("Cannot load surface!");
        let tiles = Surface::load_bmp("assets\\tiles.bmp").expect("Cannot load surface!");

        Assets {
            borders,
            numbers,
            faces,
            tiles,
        }
    }
}

impl<'a> Drawer<'a> {
    pub fn initialize_game(w: u32, h: u32) -> Drawer<'a> {
        let sdl_context = sdl2::init().unwrap_or_else(|err| {
            println!("Cannot initialize SDL! {}", err);
            std::process::exit(1);
        });

        let video_subsystem = sdl_context
            .video()
            .expect("Cannot initialize video for SDL!");

        let mut window = video_subsystem
            .window("rust-sdl2 demo", (w * 16 + 8) as u32, (h * 16 + 44) as u32)
            .position_centered()
            .build()
            .expect("Cannot initialize video mode for SDL! {}");

        window
            .set_title("Minesweeper in Rust")
            .expect("Failed to set window title!");

        let event_pump = sdl_context
            .event_pump()
            .expect("Cannot initialize event pump for SDL!");

        let assets = Assets::load();

        let d = Drawer {
            // sdl_context,
            window,
            event_pump,
            assets,
        };

        d
    }

    pub fn draw_screen(&mut self, i: f64, game: &Game) {
        let mut screen = self
            .window
            .surface(&self.event_pump)
            .expect("Cannot obtain window surface!");

        let color = Color::RGB(
            (((i).cos() + 1.0) * 126.0).round() as u8,
            64,
            (((i).sin() + 1.0) * 126.0).round() as u8,
        );

        screen.fill_rect(None, color).unwrap();

        self.draw_borders(&mut screen, game);
        self.draw_field(&mut screen, game);
        self.draw_numbers(&mut screen, game);
        self.draw_face(&mut screen, game);

        screen.finish().unwrap();
    }

    fn draw_borders(self: &Self, screen: &mut sdl2::video::WindowSurfaceRef, game: &Game) {
        let (w, h) = screen.size();
        let (w, h) = (w as i32, h as i32);
        self.assets
            .borders
            .blit(Rect::new(0, 0, 52, 40), screen, Rect::new(0, 0, 52, 40))
            .unwrap();

        self.assets
            .borders
            .blit(
                Rect::new(106, 0, 52, 40),
                screen,
                Rect::new(w - 52, 0, 52, 40),
            )
            .unwrap();

        self.assets
            .borders
            .blit(Rect::new(0, 60, 4, 4), screen, Rect::new(0, h - 4, 4, 4))
            .unwrap();
        self.assets
            .borders
            .blit(
                Rect::new(24, 60, 4, 4),
                screen,
                Rect::new(w - 4, h - 4, 4, 4),
            )
            .unwrap();

        self.assets
            .borders
            .blit(
                Rect::new(72, 0, 32, 40),
                screen,
                Rect::new(w / 2 - 16, 0, 32, 40),
            )
            .unwrap();

        for y in 0..game.height() {
            let y = y as i32;
            self.assets
                .borders
                .blit(
                    Rect::new(0, 42, 4, 16),
                    screen,
                    Rect::new(0, 40 + y * 16, 4, 16),
                )
                .unwrap();
            self.assets
                .borders
                .blit(
                    Rect::new(24, 42, 4, 16),
                    screen,
                    Rect::new(w - 4, 40 + y * 16, 4, 16),
                )
                .unwrap();
        }

        for x in 0..game.width() {
            let x = x as i32;
            self.assets
                .borders
                .blit(
                    Rect::new(6, 60, 16, 4),
                    screen,
                    Rect::new(4 + x * 16, h - 4, 16, 4),
                )
                .unwrap();
        }

        let rem = w / 2 - 52 - 32 / 2;
        for x in 0..rem / 16 {
            let x = x as i32;
            self.assets
                .borders
                .blit(
                    Rect::new(54, 0, 16, 40),
                    screen,
                    Rect::new(52 + x * 16, 0, 16, 40),
                )
                .unwrap();

            self.assets
                .borders
                .blit(
                    Rect::new(54, 0, 16, 40),
                    screen,
                    Rect::new(w / 2 + 16 + x * 16, 0, 16, 40),
                )
                .unwrap();
        }
    }

    fn draw_field(&self, screen: &mut sdl2::video::WindowSurfaceRef, game: &Game) {
        let field = game.get_field();

        for (x, row) in field.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let rect = match (&tile.revealed, &tile.flagged, &tile.content) {
                    (true, false, TileContent::Bomb) => Rect::new(18, 18, 16, 16),
                    (true, true, TileContent::Empty(_)) => Rect::new(36, 36, 16, 16),
                    (true, false, TileContent::Empty(i)) => Rect::new(i * 18, 0, 16, 16),
                    (false, false, TileContent::Bomb) => {
                        if matches!(game.state(), GameState::Playing) {
                            Rect::new(0, 36, 16, 16)
                        } else {
                            Rect::new(0, 18, 16, 16)
                        }
                    }
                    (false, false, _) => {
                        if game.preview_at(x, y) {
                            Rect::new(0, 0, 16, 16)
                        } else {
                            Rect::new(0, 36, 16, 16)
                        }
                    }

                    (false, true, _) => Rect::new(18, 36, 16, 16),
                    (_, _, _) => Rect::new(36, 18, 16, 16),
                };

                self.assets
                    .tiles
                    .blit(
                        rect,
                        screen,
                        Rect::new(x as i32 * 16 + 4, y as i32 * 16 + 40, 16, 16),
                    )
                    .unwrap();
            }
        }
    }

    fn draw_numbers(&self, screen: &mut sdl2::video::WindowSurfaceRef, game: &Game) {
        self.draw_number(screen, 35, 9, game.mines_remaining() as i32);
        self.draw_number(screen, (screen.width() - 19) as i32, 9, game.time());
    }

    fn draw_number(&self, screen: &mut sdl2::video::WindowSurfaceRef, x: i32, y: i32, number: i32) {
        let mut number = number;
        for i in 0..3 {
            let n = number % 10;
            number /= 10;

            self.assets
                .numbers
                .blit(
                    Rect::new(n * 12, 0, 10, 18),
                    screen,
                    Rect::new(x - i * 13, y, 10, 18),
                )
                .unwrap();
        }
    }

    fn draw_face(&self, screen: &mut sdl2::video::WindowSurfaceRef, game: &Game) {
        let offset = match game.state() {
            GameState::Boom(_) => 2 * 24,
            GameState::Victory(_) => 1 * 24,
            GameState::Playing => {
                if game.preview() {
                    3 * 24
                } else {
                    0
                }
            }
        };

        let w = screen.width() as i32;
        self.assets
            .faces
            .blit(
                Rect::new(offset, 0, 22, 22),
                screen,
                Rect::new(w / 2 - 11, 7, 22, 22),
            )
            .unwrap();
    }

    pub fn handle_events(&mut self, game: &mut Game) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    x,
                    y,
                    ..
                } => {
                    let x = (x - 4) / 16;
                    let y = (y - 40) / 16;
                    if x >= 0 && x < game.width() as i32 && y >= 0 && y < game.height() as i32 {
                        game.flag_tile(x as usize, y as usize);
                    }
                }
                _ => {}
            }
        }
        false
    }
}
