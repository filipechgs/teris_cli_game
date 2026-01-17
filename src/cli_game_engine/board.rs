pub mod board {
    use crate::cli_game_engine::shapes::shapes::Shape;
    use crate::cli_game_engine::types::types::Matrix4x4;
    use rand;

    pub struct GameBoard {
        pieces: [Shape; 7],
        pub board_20x28: [[u8; 20]; 28],
        pub active_piece: Shape,
        pub next_piece: Shape,
        pub piece_start_position_in_board_y_x: [usize; 2],
        is_first_piece: bool,
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
                next_piece: Shape::new_i(),
                piece_start_position_in_board_y_x: [0, 8],
                is_first_piece: true,
            }
        }

        pub fn display(&self) {
            clearscreen::clear().expect("Failed to clear screen");

            let next_piece = self.next_piece.get_current_shape();

            println!(" ____________________ ");

            for (_row_i, row) in next_piece.iter().enumerate() {
                let line_slice: &str =
                    std::str::from_utf8(row).expect("Erro: O array de bytes não é UTF-8 válido.");
                println!("|        {}        |", line_slice);
            }
            println!(" ¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨ ");

            println!(" ____________________ ");

            for line in self.board_20x28 {
                // Transforma cada array em uma string, sem separação entre os caracteres
                let line_slice: &str =
                    std::str::from_utf8(&line).expect("Erro: O array de bytes não é UTF-8 válido.");
                println!("|{}|", line_slice);
            }
            println!(" ¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨ ");
        }

        fn get_random_piece(&self) -> Shape {
            self.pieces[rand::random_range(0..self.pieces.len())]
        }

        pub fn select_new_piece(&mut self) {
            if self.is_first_piece {
                self.active_piece = self.get_random_piece();
                self.next_piece = self.get_random_piece();
                self.is_first_piece = false;
            }

            self.active_piece = self.next_piece;
            self.next_piece = self.get_random_piece()
        }

        pub fn add_piece(&mut self) {
            self.piece_start_position_in_board_y_x = [0, 8];
            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();

            for (piece_row_i, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_i, &piece_cell) in piece_line.iter().enumerate() {
                    let board_row = self.piece_start_position_in_board_y_x[0] + piece_row_i;
                    let board_col = self.piece_start_position_in_board_y_x[1] + piece_col_i;

                    if piece_cell == 35 && self.board_20x28[board_row][board_col] == 32 {
                        self.board_20x28[board_row][board_col] = 35
                    }
                }
            }
        }

        pub fn shape_fall(&mut self) -> bool {
            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();
            let piece_last_char_position_y_x: [usize; 2] =
                self.active_piece.get_last_char_position_y_x();

            // Itera sobre a peça posicionada no tabuleiro e identifica se ela pode cair
            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    if piece_cell != 35 {
                        continue;
                    }

                    let next_board_row =
                        self.piece_start_position_in_board_y_x[0] + piece_row_index + 1;

                    if next_board_row >= 28 {
                        return false;
                    }

                    let board_col = self.piece_start_position_in_board_y_x[1] + piece_col_index;

                    // Paraliza se: essa for a última linha da figura e a próxima linha do tabuleiro tiver um caractere 35 embaixo de algum caractere da figura
                    if piece_row_index >= piece_last_char_position_y_x[0]
                        && self.board_20x28[next_board_row][board_col] == 35
                    {
                        return false;
                    }
                }
            }

            // Apaga a peça na posição antiga
            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    // Evitar apagamento de outras peças lateráis
                    if piece_row_index > 3 && piece_cell == 32 {
                        continue;
                    }

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

            // Desenha a peça uma casa abaixo
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
