mod game;
mod map;
mod object;

use game::Game;

fn main() {
    Game::new().run();
}
