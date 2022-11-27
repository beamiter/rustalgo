struct  Solution;

use std::collections::HashSet;

impl Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        for i in 0..9 {
            for j in 0..9 {
                let c = board[i][j];
                if c == '.' {
                    continue;
                }
                if !rows[i].insert(c) {
                    return false;
                }
                if !cols[j].insert(c) {
                    return false;
                }
                let k = (i / 3) * 3 + (j / 3);
                if !boxes[k].insert(c) {
                    return false;
                }
            }
        }
        true
    }
}
