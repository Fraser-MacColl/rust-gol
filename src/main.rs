use crate::gol::GameOfLife;

mod gol;

fn main() {

    let game = GameOfLife::new();

    game.debug_print();
}
