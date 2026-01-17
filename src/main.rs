pub mod cli_game_engine;
use crate::cli_game_engine::board::board::GameBoard;
use std::thread;
use std::time::Duration;

fn main() {
    let mut game_board = GameBoard::new_20x28();
    game_board.add_piece(); 
  
    loop {
        if !game_board.piece_fall() { 
            game_board.select_new_piece();
            
            if !game_board.add_piece() {
                println!("GAME OVER!");
                break;
            } 
        }
        
        game_board.display();
        thread::sleep(Duration::from_millis(100));
    }
}
