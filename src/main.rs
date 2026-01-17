pub mod cli_game_engine;
use crate::cli_game_engine::board::board::GameBoard;

// Crossterm imports for keyboard handling
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Enable raw mode for immediate key press detection
    terminal::enable_raw_mode()?;
    
    let mut game_board = GameBoard::new_20x28();
    
    // Add the first piece
    game_board.select_new_piece();
    game_board.add_piece();
    
    // Main game loop
    'game_loop: loop {
        // Check for keyboard input with a timeout
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Left => {
                        game_board.move_piece_sideways(-1);
                    }
                    KeyCode::Right => {
                        game_board.move_piece_sideways(1);
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        println!("Game exited by user.");
                        break 'game_loop;
                    }
                    _ => {}
                }
            }
        }
        
        // Game logic - make piece fall
        if !game_board.piece_fall() {
            game_board.select_new_piece();
            
            if !game_board.add_piece() {
                println!("GAME OVER!");
                break 'game_loop;
            }
        }
        
        // Display the game board
        game_board.display();
        
        // Control game speed
        thread::sleep(Duration::from_millis(100));
    }
    
    // Disable raw mode before exiting
    terminal::disable_raw_mode()?;
    Ok(())
}