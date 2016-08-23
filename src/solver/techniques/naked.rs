use game::{Game};
use util::*;

pub fn solve_cell_naked_single(game: &mut Game, row: usize, col: usize) {
    if game.board[row][col].rem_poss == 1 {
        for i in 0..game.size {
            if game.board[row][col].val_poss_set[i] == true {
                set_value(game, row, col, i + 1);
            }
        }
    }
}
