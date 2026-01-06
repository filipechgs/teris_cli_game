pub mod board {
    use crate::cli_game_engine::shapes::shapes::Shape;
    use crate::cli_game_engine::types::types::Matrix4x4;
    use rand;

    pub struct GameBoard {
        pieces: [Shape; 7],
        pub board_20x28: [[u8; 20]; 28],
        pub active_piece: Shape,
        pub piece_start_position_in_board_y_x: [usize; 2],
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
                    Shape::new_j(),
                ],
                board_20x28: [line; BOARD_HEIGHT],
                active_piece: Shape::new_i(),
                piece_start_position_in_board_y_x: [0, 8],
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
            self.piece_start_position_in_board_y_x = [0, 8];
            self.active_piece = self.pieces[rand::random_range(0..self.pieces.len())];

            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();

            for (_piece_line_i, piece_line) in piece_shape.iter().enumerate() {
                for (_piece_cell_i, &piece_cell) in piece_line.iter().enumerate() {
                    let board_row = self.piece_start_position_in_board_y_x[0];
                    let board_col = self.piece_start_position_in_board_y_x[1];

                    if piece_cell == 35 && self.board_20x28[board_row][board_col] == 32 {
                        self.board_20x28[board_row][board_col] = 35
                    }
                }
            }
        }

        pub fn shape_fall(&mut self) -> bool {
            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();
            let mut piece_last_char_position_y_x: [usize; 2] = [0, 0];

            // Itera na matriz de Shape que possui o desenho da peça e identifica a posição do último caractere 35
            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                let mut is_empty_line: bool = true;

                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    if piece_cell == 35 {
                        piece_last_char_position_y_x = [piece_row_index, piece_col_index];
                        is_empty_line = false;
                    }
                }

                if is_empty_line { break; }
            }

            // Itera sobre a peça posicionada no tabuleiro e identifica se ela pode cair
            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    if piece_cell != 35 {
                        continue;
                    }

                    let next_board_row = self.piece_start_position_in_board_y_x[0] + piece_row_index + 1;
                    
                    if next_board_row >= 28 {
                        return false;
                    }

                    let board_col = self.piece_start_position_in_board_y_x[1] + piece_col_index;
                    
                    // Paraliza se: essa for a última linha da figura e a próxima linha do tabuleiro tiver um caractere 35 embaixo de algum caractere da figura
                    if  piece_row_index >= piece_last_char_position_y_x[0] && self.board_20x28[next_board_row][board_col] == 35 {
                        return false;
                    }
                }
            }

            // Apaga a peça na posição antiga
            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    // Evitar apagamento de outras peças lateráis
                    if piece_row_index > 3 && piece_cell == 32 {continue;}

                    if piece_row_index > piece_last_char_position_y_x[0] {
                        break;
                    }

                    let board_row = self.piece_start_position_in_board_y_x[0] + piece_row_index;
                    let board_col = self.piece_start_position_in_board_y_x[1] + piece_col_index;

                    if self.board_20x28[board_row][board_col] == 35 {
                        self.board_20x28[board_row][board_col] = 32
                    }
                }
            }

            // Move para baixo
            self.piece_start_position_in_board_y_x[0] += 1;

            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {

                    let board_row = self.piece_start_position_in_board_y_x[0] + piece_row_index;
                    let board_col = self.piece_start_position_in_board_y_x[1] + piece_col_index;
                    
                    if board_row < 28 {
                        if self.board_20x28[board_row][board_col] == 32 && piece_cell == 35 {
                            // Esceve a célula da figura em queda
                            self.board_20x28[board_row][board_col] = 35
                        }                    
                    }
                }
            }
            
            return true;
        }
    }
}
