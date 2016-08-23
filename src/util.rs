use game::Game;

pub fn set_value ( game: &mut Game, row: usize, col: usize, value: usize ) {
    // set the value
    game.board[row][col].value = value;
    // clear the val_poss_set for the cells
    // TODO maybe use game.size instead?
    game.board[row][col].val_poss_set = vec![false; game.board[row][col].val_poss_set.len()];
    game.board[row][col].rem_poss = 0;
    // update partner cells
    // ---- while updating, check to see if partner is now a naked solve_cell_naked_single
    game.get_row(row).remove_poss( game, value );
    game.get_col(col).remove_poss( game, value );
    game.get_subgrid(row, col).remove_poss( game, value );

    game.solved_count += 1;
}