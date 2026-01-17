pub mod cli_game_engine;
use crate::cli_game_engine::board::board::GameBoard;
use std::thread;
use std::time::Duration;

fn main() {
    // Cria tabuleiro do jogo
    let mut game_board = GameBoard::new_20x28();
    game_board.select_new_piece();
    game_board.add_piece();
    game_board.display();
  
    loop {
        if !game_board.shape_fall() { 
            game_board.select_new_piece();
            game_board.add_piece(); 
        }
        
        game_board.display();
        thread::sleep(Duration::from_millis(200));
    }
}
