pub enum GameState {
    Playing,
    Victory(u32),
    Boom(i32),
}

pub enum TileContent {
    Empty(i32),
    Bomb,
}
pub enum TileState {
    Hidden,
    Revealed,
    Flagged,
}

// #[derive(Copy, Clone)]
pub struct Tile {
    pub content: TileContent,
    pub revealed: bool,
    pub flagged: bool,
}

pub struct Game {
    w: u8,
    h: u8,
    mine_count: u16,
    state: GameState,
    field: Vec<Vec<Tile>>,
}

impl Game {
    pub fn new(w: u8, h: u8, mine_count: u16) -> Game {
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

        Game {
            w,
            h,
            mine_count,
            state: GameState::Playing,
            field,
        }
    }

    pub fn width(&self) -> u8 {
        self.w
    }
    pub fn height(&self) -> u8 {
        self.h
    }

    pub fn size(&self) -> (u8, u8) {
        (self.w, self.h)
    }

    pub fn mines_remaining(&self) -> u16 {
        999
    }

    pub fn time(&self) -> i32 {
        888
    }

    pub fn preview(&self) -> bool {
        false
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn get_field(&self) -> &Vec<Vec<Tile>> {
        &self.field
    }
}
