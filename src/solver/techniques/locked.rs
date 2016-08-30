use game::Game;
use house::{House,HouseType};

/*
    This technique is in addition to naked and hidden techniques
    If 2 to n spaces are in multiple houses, then we can reduce possibilities from both houses at the same time
*/

pub fn solve_locked( game: &mut Game, set: &(Vec<usize>,Vec<(usize,usize)>), house_type: &HouseType ) {
    let mut locked = false;
    let mut house: House = game.get_row(0);  // Rust forces us to initialize variables to something before use

    match house_type {
        &HouseType::ROW | &HouseType::COL => {
            // do spaces share a subgrid?
            let mut in_subgrid = true;
            let min_row = (set.1[0].0 / game.n)*game.n;
            let min_col = (set.1[0].1 / game.n)*game.n;
            for s in 1..set.1.len() {
                let s_min_row = (set.1[s].0 / game.n)*game.n;
                let s_min_col = (set.1[s].1 / game.n)*game.n;
                if s_min_row != min_row || s_min_col != min_col {
                    in_subgrid = false;
                    break;
                }
            }

            if in_subgrid == true {
                locked = true;
                house = game.get_subgrid( set.1[0].0, set.1[0].1 );
            }
        },
        &HouseType::SUBGRID => {
            // do spaces share a row?
            let mut in_row = true;
            for s in 1..set.1.len() {
                if set.1[s].0 != set.1[0].0 {
                    in_row = false;
                    break;
                }
            }
            if in_row == true {
                locked = true;
                house = game.get_row( set.1[0].0 );
            }
            else {
                // do spaces share a col?
                let mut in_col = true;
                for s in 1..set.1.len() {
                    if set.1[s].1 != set.1[0].1 {
                        in_col = false;
                        break;
                    }
                }

                if in_col == true {
                    locked = true;
                    house = game.get_col( set.1[0].1 );
                }
            }
        }
    }

    if locked == true {
        for v in 0..set.0.len() {
            house.remove_poss_with_exception( game, set.0[v]+1, &set.1 );
        }
    }
}