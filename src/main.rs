use crate::game_of_life::GameOfLife;

mod game_of_life;

fn main() {
    let gol = GameOfLife::new();
    gol.debug_print();
}
