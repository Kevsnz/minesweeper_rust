use rand::Rng;

pub enum GameState {
    Playing,
    Victory(u32),
    Boom(u32),
}

pub enum TileContent {
    Empty(i32),
    Bomb,
}

// #[derive(Copy, Clone)]
pub struct Tile {
    pub content: TileContent,
    pub revealed: bool,
    pub flagged: bool,
}

pub struct Game {
    w: usize,
    h: usize,
    mine_count: i32,
    state: GameState,
    field: Vec<Vec<Tile>>,
    flag_count: i32,
}

impl Game {
    pub fn new(w: usize, h: usize, mine_count: i32) -> Game {
        let field = Game::generate_field(w, h, mine_count);

        Game {
            w,
            h,
            mine_count,
            state: GameState::Playing,
            field,
            flag_count: 0,
        }
    }

    fn generate_field(w: usize, h: usize, mine_count: i32) -> Vec<Vec<Tile>> {
        let mut field: Vec<Vec<Tile>> = vec![];
        for _ in 0..w {
            let mut row = vec![];
            for _ in 0..h {
                let new_tile = Tile {
                    content: TileContent::Empty(0),
                    revealed: false,
                    flagged: false,
                };
                row.push(new_tile);
            }
            field.push(row);
        }

        let mut c = 0;
        let mut rng = rand::thread_rng();

        while c < mine_count {
            let (x, y) = (rng.gen_range(0..w) as usize, rng.gen_range(0..h) as usize);
            let tile = &mut field[x][y];
            if let TileContent::Bomb = tile.content {
                continue;
            }

            Game::place_mine(&mut field, x, x, w, h);
            c += 1;
        }

        field
    }

    fn do_for_neightbors<F: FnMut(usize, usize)>(
        x: usize,
        y: usize,
        mut func: F,
        w: usize,
        h: usize,
    ) {
        if x > 0 {
            // x - 1
            if y > 0 {
                func(x - 1, y - 1);
            }
            func(x - 1, y);
            if y < h - 1 {
                func(x - 1, y + 1);
            }
        }

        // x
        if y > 0 {
            func(x, y - 1);
        }
        if y < h - 1 {
            func(x, y + 1);
        }

        if x < w - 1 {
            // x + 1
            if y > 0 {
                func(x + 1, y - 1);
            }
            func(x + 1, y);
            if y < h - 1 {
                func(x + 1, y + 1);
            }
        }
    }

    fn add_empty(tile: &mut Tile, add: i32) {
        if let TileContent::Empty(ref mut x) = tile.content {
            *x += add;

            if *x < 0 || *x > 8 {
                panic!("Invalid value for tile neighbor!");
            }
        }
    }

    fn place_mine(field: &mut Vec<Vec<Tile>>, x: usize, y: usize, w: usize, h: usize) -> bool {
        let tile = &mut field[x][x];

        if let TileContent::Bomb = tile.content {
            panic!("Cannot place bomb - already bomb here!");
        }

        tile.content = TileContent::Bomb;
        Game::do_for_neightbors(x, y, |x, y| Game::add_empty(&mut field[x][y], 1), w, h);
        true
    }

    fn move_mine(&mut self, x: usize, y: usize) {
        Game::do_for_neightbors(
            x,
            y,
            |x, y| Game::add_empty(&mut self.field[x][y], -1),
            self.w as usize,
            self.h as usize,
        );

        let mut rng = rand::thread_rng();
        loop {
            let (new_x, new_y) = (
                rng.gen_range(0..self.w) as usize,
                rng.gen_range(0..self.h) as usize,
            );
            if new_x == x && new_y == y {
                continue;
            }
            if let TileContent::Bomb = self.field[new_x][new_y].content {
                continue;
            }

            Game::place_mine(&mut self.field, new_x, new_y, self.w, self.h);
            break;
        }

        let mut c = 0;
        Game::do_for_neightbors(
            x,
            y,
            |x, y| {
                if let TileContent::Bomb = self.field[x][y].content {
                    c += 1;
                }
            },
            self.w as usize,
            self.h as usize,
        );
        self.field[x][y].content = TileContent::Empty(c);
    }

    pub fn width(&self) -> usize {
        self.w
    }
    pub fn height(&self) -> usize {
        self.h
    }

    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn mines_remaining(&self) -> i32 {
        std::cmp::max(0, self.mine_count - self.flag_count)
    }

    pub fn time(&self) -> i32 {
        888
    }

    pub fn preview(&self) -> bool {
        false
    }

    pub fn preview_at(&self, x: usize, y: usize) -> bool {
        false
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn get_field(&self) -> &Vec<Vec<Tile>> {
        &self.field
    }
}
