pub mod board {
    use crate::cli_game_engine::shapes::shapes::Shape;
    use crate::cli_game_engine::types::types::Matrix4x4;

    pub struct GameBoard {
        pieces: [Shape; 7],
        pub board_20x28: [[u8; 20]; 28],
        pub active_piece: Shape,
        pub next_piece: Shape,
        pub piece_current_start_position_in_board_y_x: [usize; 2],
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
                piece_current_start_position_in_board_y_x: [0, 8],
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

            println!(" 0123456789ABCDEFGHIJ ");
            println!(" ____________________ ");

            for (l, line) in self.board_20x28.iter().enumerate() {
                // Transforma cada array em uma string, sem separação entre os caracteres
                let line_slice: &str =
                    std::str::from_utf8(line).expect("Erro: O array de bytes não é UTF-8 válido.");
                println!("|{}|{}", line_slice, l);
            }
            println!(" ¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨¨ ");
            println!("Press 'Esc' or 'q' to exit.");
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

        pub fn add_piece(&mut self) -> bool {
            self.piece_current_start_position_in_board_y_x = [0, 8];

            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();
            let piece_last_char_position_y_x: [usize; 2] =
                self.active_piece.get_current_shape_last_char_y_x();

            for (piece_row_i, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_i, &piece_cell) in piece_line.iter().enumerate() {
                    let board_row = self.piece_current_start_position_in_board_y_x[0] + piece_row_i;
                    let board_col = self.piece_current_start_position_in_board_y_x[1] + piece_col_i;

                    if piece_cell == 35 && self.board_20x28[board_row][board_col] == 35 {
                        return false;
                    }

                    if piece_cell == 35 && self.board_20x28[board_row][board_col] == 32 {
                        self.board_20x28[board_row][board_col] = 35
                    }
                }

                if piece_row_i >= piece_last_char_position_y_x[0] {
                    break;
                }
            }

            true
        }

        pub fn piece_fall(&mut self, piece_rotated: bool) -> bool {
            if piece_rotated {
                self.remove_piece_from_old_up_position();

                if !self.piece_can_fall() {
                    self.active_piece.undo_rotation();
                    self.draw_piece_at_aditional_y_position(0);

                    return false;
                };

                self.draw_piece_at_aditional_y_position(1);

                return true;
            }

            if self.piece_can_fall() {
                self.remove_piece_from_current_position();
                self.draw_piece_at_aditional_y_position(1);

                return true;
            }

            return false;
        }

        fn piece_can_fall(&mut self) -> bool {
            let piece_last_char_position_y_x: [usize; 2] = self.active_piece.get_current_shape_last_char_y_x();
            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();

            // dbg!(piece_last_char_position_y_x);
            // dbg!(piece_shape);

            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    if piece_cell != 35 {
                        continue;
                    }

                    let board_row = self.piece_current_start_position_in_board_y_x[0] + piece_row_index;
                    let next_board_row = board_row + 1;

                    // dbg!(piece_row_index);
                    // dbg!(board_row);

                    let is_piece_last_line = if piece_row_index == piece_last_char_position_y_x[0] {
                        true
                    } else {
                        false
                    };

                    // dbg!(is_piece_last_line);

                    if is_piece_last_line && next_board_row >= 28 {
                        return false;
                    }

                    let board_col = self.piece_current_start_position_in_board_y_x[1] + piece_col_index;

                    if is_piece_last_line {
                        if self.board_20x28[next_board_row][board_col] == 35 {
                            return false;
                        }
                    }
                }
            }

            return true;
        }

        fn remove_piece_from_old_up_position(&mut self) {
            let piece_last_char_y_x: [usize; 2] = self.active_piece.get_old_shape_last_char_y_x();
            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();

            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {

                    // Evitar apagamento de outras peças lateráis
                    if piece_row_index > 3 && piece_cell == 32 {
                        continue;
                    }

                    if piece_row_index > piece_last_char_y_x[0] {
                        break;
                    }

                    let board_row = self.piece_current_start_position_in_board_y_x[0] + piece_row_index;
                    let board_col = self.piece_current_start_position_in_board_y_x[1] + piece_col_index;

                    if board_col < 20 {
                        if self.board_20x28[board_row][board_col] == 35 {
                            self.board_20x28[board_row][board_col] = 32
                        }
                    }
                }
            }
        }

        fn draw_piece_at_aditional_y_position(&mut self, aditional_y_position: usize) {
            let piece_shape: Matrix4x4 = self.active_piece.get_current_shape();
            
            self.piece_current_start_position_in_board_y_x[0] += aditional_y_position;

            for (piece_row_index, piece_line) in piece_shape.iter().enumerate() {
                for (piece_col_index, &piece_cell) in piece_line.iter().enumerate() {
                    let board_row = self.piece_current_start_position_in_board_y_x[0] + piece_row_index;
                    let board_col = self.piece_current_start_position_in_board_y_x[1] + piece_col_index;

                    if board_row < 28 && board_col < 20 {
                        if self.board_20x28[board_row][board_col] == 32 && piece_cell == 35 {
                            // Esceve a célula da figura em queda
                            self.board_20x28[board_row][board_col] = 35
                        }
                    }
                }
            }
        }

        pub fn rotate_piece(&mut self) -> bool {
            self.active_piece.rotate();

            return true;
        }

        pub fn move_piece_sideways(&mut self, direction: i32) -> bool {
            let new_x = (self.piece_current_start_position_in_board_y_x[1] as i32 + direction) as usize;

            // Basic boundary check
            if direction < 0 && self.piece_current_start_position_in_board_y_x[1] <= 0 {
                return false; // Can't move left if at left edge
            }

            let piece_last_char_y_x = self.active_piece.get_current_shape_last_char_y_x();
            if direction > 0 && self.piece_current_start_position_in_board_y_x[1] >= (20 - piece_last_char_y_x[1]) {
                return false; // Can't move right if piece would go off board (20 - 4 = 16)
            }

            self.remove_piece_from_current_position();

            // Try new position
            let old_x = self.piece_current_start_position_in_board_y_x[1];
            self.piece_current_start_position_in_board_y_x[1] = new_x;

            let is_valid_position = self.is_current_position_valid();

            if !is_valid_position {
                self.piece_current_start_position_in_board_y_x[1] = old_x;
            }

            // Redraw piece
            self.draw_piece_at_current_position();

            is_valid_position
        }

        fn remove_piece_from_current_position(&mut self) {
            let piece_shape = self.active_piece.get_current_shape();
            let last_pos = self.active_piece.get_current_shape_last_char_y_x();

            for row in 0..=last_pos[0] {
                for col in 0..4 {
                    if piece_shape[row][col] == 35 {
                        let board_row = self.piece_current_start_position_in_board_y_x[0] + row;
                        let board_col = self.piece_current_start_position_in_board_y_x[1] + col;

                        if board_row < 28 && board_col < 20 {
                            self.board_20x28[board_row][board_col] = 32;
                        }
                    }
                }
            }
        }

        fn draw_piece_at_current_position(&mut self) {
            let piece_shape = self.active_piece.get_current_shape();
            let last_pos = self.active_piece.get_current_shape_last_char_y_x();

            for row in 0..=last_pos[0] {
                for col in 0..4 {
                    if piece_shape[row][col] == 35 {
                        let board_row = self.piece_current_start_position_in_board_y_x[0] + row;
                        let board_col = self.piece_current_start_position_in_board_y_x[1] + col;

                        if board_row < 28 && board_col < 20 {
                            self.board_20x28[board_row][board_col] = 35;
                        }
                    }
                }
            }
        }

        fn is_current_position_valid(&self) -> bool {
            // BUG - verificar se a próxima posição, e não a atual, é válida
            let piece_shape = self.active_piece.get_current_shape();
            let last_pos = self.active_piece.get_current_shape_last_char_y_x();

            for row in 0..=last_pos[0] {
                for col in 0..4 {
                    if piece_shape[row][col] == 35 {
                        let board_row = self.piece_current_start_position_in_board_y_x[0] + row;
                        let board_col = self.piece_current_start_position_in_board_y_x[1] + col;

                        if board_row >= 28 || board_col >= 20 {
                            return false;
                        }

                        // Check collision with existing pieces
                        // We need to be careful here - we should check if the cell
                        // is occupied by something other than our current piece
                        if self.board_20x28[board_row][board_col] == 35 {
                            // This is a simplified check - in a full implementation,
                            // we'd track which pieces are where
                            return false;
                        }
                    }
                }
            }
            true
        }

        pub fn clear_board_line(&mut self) {
            let board = self.board_20x28;
            let mut line_replaced = false;

            for (y, row ) in board.iter().enumerate().rev() {
                let mut filled_line: bool = false;
                let mut empty_line: bool = true;
                let mut char_counter: usize = 0;

                for (_c, char) in row.iter().enumerate() {
                    if *char == 35 {
                        empty_line = false;
                        char_counter += 1;
                    }

                    if char_counter == 20 {
                        filled_line = true;
                    }
                }

                if empty_line && line_replaced {
                    for (c, _char) in row.iter().enumerate() {
                        self.board_20x28[y][c] = 32;
                        // Copia a linha logo acima para a posição atual
                        self.board_20x28[y][c] = self.board_20x28[y-1][c];
                        // Transforma linha logo acima em em linha em branco;
                        self.board_20x28[y-1][c] = 32; 
                    }
                }

                // Remove caracteres '#', 35, da linha preenchida
                for (c, _char) in row.iter().enumerate() {
                    if filled_line {
                        self.board_20x28[y][c] = 32;
                        // Copia a linha logo acima para a posição atual
                        self.board_20x28[y][c] = self.board_20x28[y-1][c];
                        // Transforma linha logo acima em em linha em branco;
                        self.board_20x28[y-1][c] = 32; 
                        // Sinaliza que linha logo acima foi apagada;
                        line_replaced = true;
                    }
                }

                if empty_line {
                    break;
                }
            }
            

        }

    }
}
