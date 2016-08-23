use game::Game;
use solver::techniques::naked::solve_cell_naked_single;

pub enum HouseType {
    ROW,
    COL,
    SUBGRID,
}

pub struct House {
    pub house_type: HouseType,
    pub spaces: Vec<(usize, usize)>,
}

impl House {
    pub fn remove_poss(&mut self, game: &mut Game, val: usize) {
        for i in 0..self.spaces.len() {
            if game.board[self.spaces[i].0][self.spaces[i].1].value != 0 {
                continue;
            }
            if game.board[self.spaces[i].0][self.spaces[i].1].val_poss_set[val - 1] == true {
                game.board[self.spaces[i].0][self.spaces[i].1].val_poss_set[val - 1] = false;
                game.board[self.spaces[i].0][self.spaces[i].1].rem_poss -= 1;
            }
            // check for and solve naked singles
            solve_cell_naked_single(game, self.spaces[i].0, self.spaces[i].1);
        }
    }

    pub fn remove_poss_with_exception(&mut self,
                                      game: &mut Game,
                                      val: usize,
                                      space_list: &Vec<(usize, usize)>) {
        for i in 0..self.spaces.len() {
            let mut found = false;
            for s in 0..space_list.len() {
                if self.spaces[i].0 == space_list[s].0 && self.spaces[i].1 == space_list[s].1 {
                    found = true;
                    break;
                }
            }
            if found == true {
                continue;
            }
            // TODO: replace the following with self.remove_poss ???
            if game.board[self.spaces[i].0][self.spaces[i].1].value != 0 {
                continue;
            }
            if game.board[self.spaces[i].0][self.spaces[i].1].val_poss_set[val - 1] == true {
                game.board[self.spaces[i].0][self.spaces[i].1].val_poss_set[val - 1] = false;
                game.board[self.spaces[i].0][self.spaces[i].1].rem_poss -= 1;
            }
            // check for and solve naked singles
            solve_cell_naked_single(game, self.spaces[i].0, self.spaces[i].1);
        }
    }
}
