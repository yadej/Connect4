use ratatui::text::{Span, Line};

pub const ROWS: u16 = 6;
pub const COLS: u16 = 7;

#[derive(Debug, Clone, PartialEq)]
pub enum Tile{
    RED,
    YELLOW,
    NONE,
} 

#[derive(Debug, Clone)]
pub struct Grid{
    pub grid: Vec<Vec<Tile>>,
    pub current_winner: Tile,
    pub current_cursor: u16,
    pub current_player: Tile,
} 

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: vec![vec![Tile::NONE;COLS.into() ]; ROWS.into()],
            current_winner: Tile::NONE,
            current_cursor: 3,
            current_player: Tile::YELLOW,
        }
    }

    pub fn change_player(&mut self){
        match self.current_player {
            Tile::YELLOW => self.current_player = Tile::RED,
            _ => self.current_player = Tile::YELLOW,
        }
    }
    
    pub fn change_col(&mut self, value: i8){
        match value {
            -1 => {
                if self.current_cursor > 0{
                    self.current_cursor -= 1;
                }
            },
            1 => {
                if self.current_cursor < COLS - 1 {
                    self.current_cursor += 1;
                }
            },
            _ => panic!("You can't go out of the grid"),
        }
    }

    pub fn tile_to_ascii(&self, tile: Tile) -> Vec<Line> {
        match tile {
            Tile::NONE => vec![
                Line::from(Span::raw("         ")),
                Line::from(Span::raw("         ")),
                Line::from(Span::raw("         ")),
                Line::from(Span::raw("         ")),
            ],
            _ => vec![
                Line::from(Span::raw("  ----   ")),
                Line::from(Span::raw(" |----|  ")),
                Line::from(Span::raw(" |----|  ")),
                Line::from(Span::raw("  ----   ")),
            ],
        }
    }

    pub fn is_valid_move(&self, col: u16) -> bool {
        col < COLS && self.grid[0][col as usize] == Tile::NONE
    }

    pub fn drop_piece(&mut self, col: u16) {
        if self.is_valid_move(col){
            for row in (0..ROWS).rev(){
                if self.grid[row as usize][col as usize] == Tile::NONE {
                    self.grid[row as usize][col as usize]  = self.current_player.clone();
                    if self.check_win(row, col, self.current_player.clone()){
                        self.current_winner = self.current_player.clone();
                    }
                    self.change_player();
                    break;
                }
            }
        }
    }
    
    pub fn check_win(&self, row: u16, col: u16, player: Tile) -> bool {
        let directions = [(0, 1), (1, 0), (1, 1), (-1, 1)];
        for &(dr, dc) in &directions {
            let mut count = 1;
            for &mult in &[-1, 1] {
                let mut r = row as isize + dr * mult;
                let mut c = col as isize + dc * mult;
                while r >= 0 && r < ROWS as isize && c >= 0 && c < COLS as isize &&
                      self.grid[r as usize][c as usize] == player {
                    count += 1;
                    r += dr * mult;
                    c += dc * mult;
                }
            }
            if count >= 4 {
                return true;
            }
        }
        false
    }

    pub fn is_game_end(&mut self) -> bool {
        if self.current_winner != Tile::NONE{
            return true;
        }
        for i in 0..COLS {
            if self.is_valid_move(i) {
                return false;
            }
        }

        true
    }

    pub fn reload_grid(&mut self){
        if self.is_game_end() {
            *self = Grid::new();
        }
    }
}
