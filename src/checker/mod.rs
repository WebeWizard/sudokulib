use game::Game;
use house::House;

pub fn check( game: &Game ) -> bool {
    // each row, column, and subgrid need to have each value 1 to game.size
    let mut valid = false;
    for r in 0..game.size {
        valid = check_house( &game, game.get_row(r) );
        if valid == false { return false; }
    }

    for c in 0..game.size {
        valid = check_house( &game, game.get_col(c) );
        if valid == false { return false; }
    }

    for r in 0..game.n {
        for c in 0..game.n {
            valid = check_house( &game, game.get_subgrid(r*game.n, c*game.n) );
            if valid == false { return false; }
        }
    }
    return valid;
}

// checks that a house has values from 1 to game.size
pub fn check_house( game: &Game, house: House ) -> bool {
    let mut check = true;
    for v in 1..game.size+1 {
        let mut found = false;
        for i in 0..game.size {
            if game.board[house.spaces[i].0][house.spaces[i].1].value == v {
                found = true;
                break;
            }
        }
        if found == false {
            check = false;
            break;
        }
    }
    return check;
}