pub mod cli_game_engine;
use crate::cli_game_engine::board::board::GameBoard;

// Crossterm imports for keyboard handling
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::collections::HashSet;
use std::io;
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    // Enable raw mode for immediate key press detection
    terminal::enable_raw_mode()?;

    let mut game_board = GameBoard::new_20x28();
    game_board.select_new_piece();
    game_board.add_piece();

    let mut pressed_keys: HashSet<KeyCode> = HashSet::new();
    
    'game_loop: loop {
        game_board.display();
        
        let mut piece_rotated: bool = false;

        thread::sleep(Duration::from_millis(185));

        // Utiliza o while para drenar os eventos acumulados no buffer
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                
                // Processa a ação apenas quando a tecla é pressionada pela primeira vez
                if kind == event::KeyEventKind::Press && pressed_keys.insert(code) {
                    match code {
                        KeyCode::Up => {
                            piece_rotated = game_board.rotate_piece();
                        }
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
                } else if kind == event::KeyEventKind::Release {
                    pressed_keys.remove(&code);
                }
            }
        }
        
        dbg!(piece_rotated);

        let piece_can_fall = game_board.piece_fall(piece_rotated);
        
        dbg!(piece_can_fall);

        if !piece_can_fall {
            game_board.clear_board_line();
            game_board.select_new_piece();

            if !game_board.add_piece() {
                println!("GAME OVER!");
                break 'game_loop;
            }
        }

    }

    terminal::disable_raw_mode()?;
    Ok(())
}
