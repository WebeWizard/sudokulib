pub mod techniques;

use game::Game;
use self::techniques::hidden::solve_game_hidden;
use self::techniques::chains::solve_game_chains;

pub fn solve( game: &mut Game ) {
    while game.solved_count < game.total {
        solve_game_hidden(game);
        println!("{}",game);
        // TODO: finish chains technique
        //solve_game_chains(game);
    }
}