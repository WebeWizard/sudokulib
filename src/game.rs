use std::fmt;

use house::{House, HouseType};
use util::set_value;

// Our game is laid out with position 0,0 being the top left of the board
// 0 1 2 | 3 4 5 | 6 7 8
// 0 x x x | x x x | x x x
// 1 x x x | x x x | x x x
// 2 x x x | x x x | x x x
// ------------------------
// 3 x x x | x x x | x x x
// 4 x x x | x x x | x x x
// 5 x x x | x x x | x x x
// ------------------------
// 6 x x x | x x x | x x x
// 7 x x x | x x x | x x x
// 8 x x x | x x x | x x x
//

#[derive(Debug)]
pub struct Game {
    pub board: Vec<Vec<Space>>,
    pub n: usize,
    pub size: usize,
    pub total: usize,
    pub solved_count: usize,
}

#[derive(Clone, Debug)]
pub struct Space {
    pub value: usize,
    pub val_poss_set: Vec<bool>,
    pub rem_poss: usize,
    pub col: usize,
    pub row: usize,
}

impl Game {
    pub fn new(n: usize) -> Game {
        let size = n * n;
        let new_poss_set: Vec<bool> = vec![true; size];
        let new_space: Space = Space {
            value: 0,
            val_poss_set: new_poss_set,
            rem_poss: size,
            row: 0,
            col: 0,
        };
        let mut new_game: Game = Game {
            board: vec![vec![new_space; size]; size],
            n: n,
            size: size,
            total: size*size,
            solved_count: 0,
        };

        for row in 0..size {
            for col in 0..size {
                new_game.board[row][col].row = row;
                new_game.board[row][col].col = col;
            }
        }

        return new_game;
    }

    pub fn init_from_str(&mut self, init_vals: &str) {
        let init_vals_bytes = init_vals.as_bytes();
        for y in 0..self.size {
            for x in 0..self.size {
                let new_val =
                    (init_vals_bytes[y * self.size + x] as char).to_digit(10).unwrap() as usize;
                if new_val == 0 {
                    continue;
                }
                set_value(self, y, x, new_val);
            }
        }
    }

    pub fn get_row(&self, row: usize) -> House {
        let mut row_spaces: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.size {
            row_spaces.push((row, i));
        }
        return House {
            house_type: HouseType::ROW,
            spaces: row_spaces,
        };
    }

    pub fn get_col(&self, col: usize) -> House {
        let mut col_spaces: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.size {
            col_spaces.push((col, i));
        }
        return House {
            house_type: HouseType::COL,
            spaces: col_spaces,
        };
    }

    pub fn get_subgrid(&self, row: usize, col: usize) -> House {
        let base_row = (row / self.n) * self.n;
        let base_col = (col / self.n) * self.n;
        let mut subgrid_spaces: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.n {
            for x in 0..self.n {
                subgrid_spaces.push((base_row + y, base_col + x));
            }
        }
        return House {
            house_type: HouseType::SUBGRID,
            spaces: subgrid_spaces,
        };
    }

    pub fn get_partner_houses(&self, row: usize, col: usize) -> Vec<House> {
        let mut partners: Vec<House> = Vec::new();
        partners.push(self.get_row(row));
        partners.push(self.get_col(row));
        partners.push(self.get_subgrid(row, col));

        return partners;
    }
}

#[allow(unused_must_use)]
impl fmt::Display for Game {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f,"   ");
        for col in 0..self.size {
            if col != 0 && col != self.size && col%self.n == 0 { write!(f," |"); }
            write!(f, " {}", col);
        }
        writeln!(f,"");
        writeln!(f,"-------------------------");
        for row in 0..self.size {
            if row != 0 && row != self.size && row%self.n == 0 { writeln!(f,"-------------------------"); }
            write!(f,"{} |", row);
            for col in 0..self.size {
                if col != 0 && col != self.size && col%self.n == 0 { write!(f," |"); }
                write!(f, " {}", self.board[row][col].value);
            }
            writeln!(f,"");
        }
        writeln!(f,"")
    }
}
