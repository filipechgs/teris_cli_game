pub mod board {
    use crate::cli_game_engine::shapes::shapes::Shape;
    use crate::cli_game_engine::types::types::Matrix4x4;
    use rand;
    use std::thread;
    use std::time::Duration;

    pub struct GameBoard {
        pieces: [Shape; 7],
        pub board_20x28: [[u8; 20]; 28],
        pub active_piece: Shape,
        pub piece_board_position: [usize; 2]
    }

    impl GameBoard {
        pub fn new_20x28() -> Self {
            const LINE_WIDTH: usize = 20;
            const BOARD_HEIGHT: usize = 28;
            
            let fill_char: u8 = 32;
            let line: [u8; LINE_WIDTH] = [fill_char; LINE_WIDTH];

            GameBoard { 
                pieces: [
                    Shape::new_i(), 
                    Shape::new_o(), 
                    Shape::new_t(), 
                    Shape::new_s(), 
                    Shape::new_z(), 
                    Shape::new_l(), 
                    Shape::new_j()
                ],
                board_20x28: [line; BOARD_HEIGHT],
                active_piece: Shape::new_i(),
                piece_board_position: [0, 8] // row, col
            }
        }

        pub fn display(&self) {
            clearscreen::clear().expect("Failed to clear screen");

            for line in self.board_20x28 {
                // Transforma cada array em uma string, sem separação entre os caracteres
                let line_slice: &str =
                    std::str::from_utf8(&line).expect("Erro: O array de bytes não é UTF-8 válido.");
                println!("{:?}", line_slice);
            }
        }
        
        pub fn add_shape(&mut self) {
            self.piece_board_position = [0, 8];
            self.active_piece = self.pieces[rand::random_range(0..self.pieces.len())]; 

            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();

            for (_piece_line_i, piece_line) in piece_shape.iter().enumerate() {
                for (_piece_cell_i, &piece_cell) in piece_line.iter().enumerate() {
                
                    let board_row = self.piece_board_position[0];
                    let board_col = self.piece_board_position[1];

                    if piece_cell == 35 && self.board_20x28[board_row][board_col] == 32 {
                        self.board_20x28[board_row][board_col] = 35
                    }
                }
            }
        }
        
        pub fn shape_fall(&mut self) {
            let mut is_inside_board: bool = true; // Reveja esta condição

            loop {
                let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();

                // Pode mover para baixo?
                
                // Apaga a peça na posição antiga
                for (piece_line_index, piece_line) in piece_shape.iter().enumerate() {
                    
                    for (piece_cell_index, &piece_cell) in piece_line.iter().enumerate() {      
                        // Evita apagamento de outras peças lateráis
                        if piece_cell_index > 3 && piece_cell == 32 {continue;}
                        
                        let board_row = self.piece_board_position[0] + piece_line_index;
                        let board_col = self.piece_board_position[1] + piece_cell_index;
                   
                        is_inside_board = if board_row < self.board_20x28.len() {true} else {false};
                        
                        if is_inside_board {
                            if self.board_20x28[board_row][board_col] == 35 {
                                self.board_20x28[board_row][board_col] = 32
                            }
                        }
                    }
                }
                
                self.active_piece.rotate();

                // Move para baixo 
                self.piece_board_position[0] += 1;
                for (piece_line_index, piece_line) in piece_shape.iter().enumerate() {

                    for (piece_cell_index, &piece_cell) in piece_line.iter().enumerate() {      
                        let next_board_row = self.piece_board_position[0] + piece_line_index;
                        let next_board_col = self.piece_board_position[1] + piece_cell_index;
                        
                        // Se a p´roxima celula abaixo for 35, para o laço for externo
                        is_inside_board = if next_board_row < self.board_20x28.len() {true} else {false};
                        
                        if is_inside_board {
                         if self.board_20x28[next_board_row][next_board_col] == 32 && piece_cell == 35 {
                                // Esceve a célula da figura em queda
                                self.board_20x28[next_board_row][next_board_col] = 35
                            }
                        }
                    }

                }

                self.display();
                thread::sleep(Duration::from_millis(100));
                
                if !is_inside_board { break };
            }
            
            self.add_shape();
            self.shape_fall();

        }

    }
}