pub mod techniques;

use game::Game;
use self::techniques::hidden::solve_game_hidden;

pub fn solve( game: &mut Game ) {
    while game.solved_count < game.total {
        solve_game_hidden(game);
    }
}