pub mod shapes {
    use crate::cli_game_engine::types::types::Matrix4x4;

    #[derive(Debug, Clone, Copy)]
    pub struct Shape {
        positions: [Matrix4x4; 4],
        max_position: usize,
        pub current_position: usize,
    }

    impl Shape {
        pub fn new_i() -> Self {
            Self {
                positions: [
                    [
                        [35, 35, 35, 35],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                    ],
                    [
                        [35, 35, 35, 35],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                    ],
                ],
                max_position: 1,
                current_position: 0,
            }
        }

        pub fn new_o() -> Self {
            Self {
                positions: [
                    [
                        [32, 35, 35, 32],
                        [32, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 35, 32],
                        [32, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 35, 32],
                        [32, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 35, 32],
                        [32, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                ],
                max_position: 0,
                current_position: 0,
            }
        }

        pub fn new_t() -> Self {
            Self {
                positions: [
                    [
                        [35, 35, 35, 32],
                        [32, 35, 32, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 32, 32, 32],
                        [35, 35, 32, 32],
                        [35, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 32, 32],
                        [35, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 32, 32],
                        [35, 35, 32, 32],
                        [32, 35, 32, 32],
                        [32, 32, 32, 32],
                    ],
                ],
                max_position: 3,
                current_position: 0,
            }
        }

        pub fn new_s() -> Self {
            Self {
                positions: [
                    [
                        [32, 35, 35, 32],
                        [35, 35, 32, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 32, 32, 32],
                        [35, 35, 32, 32],
                        [32, 35, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 35, 32],
                        [35, 35, 32, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 32, 32, 32],
                        [35, 35, 32, 32],
                        [32, 35, 32, 32],
                        [32, 32, 32, 32],
                    ],
                ],
                max_position: 1,
                current_position: 0,
            }
        }

        pub fn new_z() -> Self {
            Self {
                positions: [
                    [
                        [35, 35, 32, 32],
                        [32, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 32, 32],
                        [35, 35, 32, 32],
                        [35, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 35, 32, 32],
                        [32, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 32, 32],
                        [35, 35, 32, 32],
                        [35, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                ],
                max_position: 1,
                current_position: 0,
            }
        }

        pub fn new_l() -> Self {
            Self {
                positions: [
                    [
                        [32, 32, 35, 32],
                        [35, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                        [35, 35, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 35, 35, 32],
                        [35, 32, 32, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 35, 32, 32],
                        [32, 35, 32, 32],
                        [32, 35, 32, 32],
                        [32, 32, 32, 32],
                    ],
                ],
                max_position: 3,
                current_position: 0,
            }
        }

        pub fn new_j() -> Self {
            Self {
                positions: [
                    [
                        [35, 32, 32, 32],
                        [35, 35, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [32, 35, 32, 32],
                        [32, 35, 32, 32],
                        [35, 35, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 35, 35, 32],
                        [32, 32, 35, 32],
                        [32, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                    [
                        [35, 35, 32, 32],
                        [35, 32, 32, 32],
                        [35, 32, 32, 32],
                        [32, 32, 32, 32],
                    ],
                ],
                max_position: 3,
                current_position: 0,
            }
        }

        pub fn get_last_char_position_y_x(&self) -> [usize; 2] {
            let piece_shape: Matrix4x4 = self.get_current_shape();
            let mut piece_last_char_position_y_x: [usize; 2] = [0, 0];

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

            piece_last_char_position_y_x
        }

        pub fn get_current_shape(&self) -> Matrix4x4 {
            self.positions[self.current_position]
        }

        pub fn rotate(&mut self) {
            if self.current_position < self.max_position {
                self.current_position += 1;
            } else {
                self.current_position = 0;
            }
        }
    }
}
