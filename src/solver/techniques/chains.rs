use game::Game;

// includes techniques like X-Wing and Swordfish
// -- if we can find a "chain" of spaces that have the same poss_val, then we can eliminate the poss_val
// -- from other spaces in those rows/columns ( 2 rows/cols = X-wing, 3+ = Swordfish )
// -- -- if starting with rows then we must use each space in the row that has the poss val
// -- -- then we can remove the poss_val from the spaces in the columns (that form the chains) except
// -- -- the spaces that create the chain

pub fn solve_game_chains( game: &mut Game ) {
    // for each poss_val
    // build a list of spaces that contain the poss_val (space_list)
    // see if we can find a chain from the space_list... list...

    // TODO: Starting with rows, figure out cols later

    for v in 0..game.size {
        let mut row_list: Vec<usize> = Vec::new();
        let mut space_list: Vec<Vec<(usize,usize)>> = Vec::new();
        for row in 0..game.size {
            let mut found = false;
            let mut row_space_list: Vec<(usize,usize)> = Vec::new();
            for col in 0..game.size {
                if game.board[row][col].val_poss_set[v] == true {
                    if found == false {
                        found = true;
                        row_list.push( row );
                    }
                    row_space_list.push( (row,col) );
                }
            }
            space_list.push( row_space_list );
        }
        let chain = detect_chain( (row_list, space_list) );

    }

}

fn detect_chain( space_list: (Vec<usize>,Vec<Vec<(usize,usize)>>) ) -> Vec<(usize,usize)> {
    let mut chain: Vec<(usize,usize)> = Vec::new();
    // must be atleast 2 rows
    if space_list.0.len() >= 2 {
        // TODO: meat and potatoes go here
        for i in 0..space_list.0.len() {
            // for each row, see if we can start and complete a chain

            // -- each row can have 2+ spaces with poss_val
            // -- but the connection columns MUST only have 2

            // my head hurts... might save this for another day
            // try recursing first, not entirely sure we need it.
            // if recursing doesn't work, try a while loop
        }
    }
    return chain;
}

fn chain_recurse( pos: usize, space_list: (Vec<usize>,Vec<Vec<(usize,usize)>>), mut prefix:Vec<(usize,usize)> ) {
    // TODO: only working with rows for now, do columns later

}