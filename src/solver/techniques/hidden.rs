use game::Game;
use house::House;
use super::locked::*;
use solver::techniques::naked::solve_cell_naked_single;

/*
    For each remaining poss val, build a liskt of spaces that the poss_val occurs in.
    Then, test every combination of poss_val "space list's".  If the # of "space lists" (or poss_vals) used to make up the combination
    is equal to the # of unique spaces in the combined list, then the combination is a hidden set.
*/

pub fn solve_game_hidden (game: &mut Game) {
    // for each subgrid
    for sub_y in 0..game.n {
        for sub_x in 0..game.n {
            let subgrid_house = game.get_subgrid(sub_y*game.n, sub_x*game.n);
            solve_house_hidden(game, &subgrid_house);
        }
    }

    // for each row
    for row in 0..game.size {
        let row_house = game.get_row(row);
        solve_house_hidden(game, &row_house);
    }

    // for each col
    for col in 0..game.size {
        let col_house = game.get_col(col);
        solve_house_hidden(game, &col_house);
    }
}

pub fn solve_house_hidden (game: &mut Game, house: &House) {
    // which poss_vals remain?
    let mut rem_poss_vals: Vec<usize> = Vec::new();
    let mut val_space_list: Vec<Vec<(usize,usize)>> = Vec::new();
    // poss_val 1 will be presented as 0, 2 will be 1, etc.
    for val in 0..game.size {
        let mut solved = false;
        let mut space_list: Vec<(usize,usize)> = Vec::new();
        for i in 0..house.spaces.len() {
            if game.board[house.spaces[i].0][house.spaces[i].1].value == val+1 {
                solved = true;
                break;
            }
            if game.board[house.spaces[i].0][house.spaces[i].1].val_poss_set[val] == true {
                space_list.push( (house.spaces[i].0,house.spaces[i].1) );
            }
        }
        if solved == false {
            rem_poss_vals.push(val);
            val_space_list.push(space_list);
        }
    }

    // test every combination of space_lists
    for i in 0..rem_poss_vals.len() {
        let prefix: (Vec<usize>,Vec<(usize,usize)>) = (Vec::new(),Vec::new());
        hidden_recurse( i, game, house, &rem_poss_vals, &val_space_list, prefix );
    }
}

fn hidden_recurse( pos: usize, game: &mut Game, house: &House, rem_poss_vals: &Vec<usize>, val_space_list: &Vec<Vec<(usize,usize)>>, mut prefix: (Vec<usize>,Vec<(usize,usize)>) ) {
    prefix.0.push( rem_poss_vals[pos] );

    for s in 0..val_space_list[pos].len() {
        let mut found = false;
        for i in 0..prefix.1.len() {
            if val_space_list[pos][s] == prefix.1[i] { found = true; }
        }
        if found == false { prefix.1.push(val_space_list[pos][s]); }
    }

    // is it a hidden set?
    if prefix.1.len() == prefix.0.len() && prefix.0.len() != rem_poss_vals.len() {
        // remove all other candidates from those cells
        for s in 0..prefix.1.len() {
            for v in 0..game.size {
                // is v in the prefix?
                let mut found = false;
                for val in 0..prefix.0.len() {
                    if prefix.0[val] == v {
                        found = true;
                        break;
                    }
                }
                if found == false {
                    if game.board[prefix.1[s].0][prefix.1[s].1].val_poss_set[v] == true {
                        game.board[prefix.1[s].0][prefix.1[s].1].val_poss_set[v] = false;
                        game.board[prefix.1[s].0][prefix.1[s].1].rem_poss -= 1;
                    }
                }
            }
        }
    }
    if prefix.0.len() == 1 && prefix.1.len() == 1 {
        solve_cell_naked_single( game, prefix.1[0].0, prefix.1[0].1 );
    }
    if prefix.0.len() <= game.n && prefix.1.len() <= game.n {


        solve_locked( game, &prefix, &house.house_type );
    }

    for i in pos+1..rem_poss_vals.len() {
        let newprefix = prefix.clone();
        hidden_recurse( i, game, house, rem_poss_vals, val_space_list, newprefix );
    }
}