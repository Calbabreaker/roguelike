mod game;
mod map;
mod object;

use game::Game;

fn main() {
    tcod::system::set_fps(20);
    Game::new().run();
}
