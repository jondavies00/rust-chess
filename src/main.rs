use game::Game;
use pieces::Piece;

mod game;
mod pieces;
mod board;
mod move_piece;
mod errors;
fn main() {
    let g = Game::new(None);
}
