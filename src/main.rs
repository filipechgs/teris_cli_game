pub mod cli_game_engine;
use crate::cli_game_engine::board::board::GameBoard;

fn main() {
    // Cria tabuleiro do jogo
    let mut tetris_board = GameBoard::new_20x28();
    tetris_board.add_shape();
    tetris_board.display();
    
    tetris_board.shape_fall();   
    

}
