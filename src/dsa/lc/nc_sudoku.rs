use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let ctn = |c: char| -> u8 { c.to_digit(10).unwrap() as u8 };

    let mut row_map: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut col_map: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut square_map: Vec<HashSet<char>> = vec![HashSet::new(); 9];

    for (idx_row, row) in board.iter().enumerate() {
        for (idx_col, char) in row.iter().enumerate() {
            if *char == '.' {
                continue;
            }
            if !row_map[idx_row].insert(*char) {
                return false;
            }

            if !col_map[idx_col].insert(*char) {
                return false;
            }

            let square_idx = idx_col / 3 + (idx_row / 3 * 3);
            if !square_map[square_idx].insert(*char) {
                return false;
            }
        }
    }

    true
}
