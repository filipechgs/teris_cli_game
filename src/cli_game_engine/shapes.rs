pub mod shapes {
    use crate::cli_game_engine::types::types::Matrix4x4;

    #[derive(Debug, Clone, Copy)]
    pub struct Shape {
        positions: [Matrix4x4; 4],
        max_position: usize,
        pub shape_current_position: usize,
        pub shape_old_position: usize,
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
                max_position: 3,
                shape_current_position: 0,
                shape_old_position: 0
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
                max_position: 3,
                shape_current_position: 0,
                shape_old_position: 0
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
                shape_current_position: 0,
                shape_old_position: 0
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
                max_position: 3,
                shape_current_position: 0,
                shape_old_position: 0
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
                max_position: 3,
                shape_current_position: 0,
                shape_old_position: 0
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
                shape_current_position: 0,
                shape_old_position: 0
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
                shape_current_position: 0,
                shape_old_position: 0
            }
        }

        pub fn get_old_shape_last_char_y_x(&self) -> [usize; 2] {
            let piece_shape: Matrix4x4 = self.get_old_shape();
            let mut last_char_position_y_x: [usize; 2] = [0, 0];
            
            for (piece_y, piece_line) in piece_shape.iter().enumerate() {
                let mut is_empty_line: bool = true;
                
                for (piece_x, piece_cell) in piece_line.iter().enumerate() {
                    if *piece_cell == 35 {
                        last_char_position_y_x[0] = piece_y;
                        last_char_position_y_x[1] = piece_x;
                        is_empty_line = false;
                    }
                }
                
                if is_empty_line { break; }
            }
            
            last_char_position_y_x  
        }

        pub fn get_current_shape_last_char_y_x(&self) -> [usize; 2] {
            let piece_shape: Matrix4x4 = self.get_current_shape();
            let mut last_char_position_y_x: [usize; 2] = [0, 0];
            
            for (piece_y, piece_line) in piece_shape.iter().enumerate() {
                let mut is_empty_line: bool = true;
                
                for (piece_x, piece_cell) in piece_line.iter().enumerate() {
                    if *piece_cell == 35 {
                        last_char_position_y_x[0] = piece_y;
                        last_char_position_y_x[1] = piece_x;
                        is_empty_line = false;
                    }
                }
                
                if is_empty_line { break; }
            }
            
            last_char_position_y_x  
        }

        pub fn get_current_shape(&self) -> Matrix4x4 {
            self.positions[self.shape_current_position]
        }
        
        pub fn get_old_shape(&self) -> Matrix4x4 {
            self.positions[self.shape_old_position]
        }

        pub fn rotate(&mut self) {
            self.shape_old_position = self.shape_current_position;

            if self.shape_current_position < self.max_position {
                self.shape_current_position += 1;    
            }
            else {
                self.shape_current_position = 0;
            }
        }

        pub fn undo_rotation(&mut self) {
            
            self.shape_current_position = self.shape_old_position;
        }
    }
}
