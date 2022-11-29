struct Solution;

impl Solution {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut queens: Vec<u32> = vec![];
        let mut res = vec![];
        let mut column: u32 = 0;
        let mut diagonal: u32 = 0;
        let mut diagona2: u32 = 0;
        Self::dfs(
            0,
            &mut queens,
            &mut column,
            &mut diagonal,
            &mut diagona2,
            &mut res,
            n,
        );
        res
    }

    fn dfs(
        i: usize,
        queens: &mut Vec<u32>,
        column: &mut u32,
        diagonal: &mut u32,
        diagona2: &mut u32,
        all: &mut Vec<Vec<String>>,
        n: usize,
    ) {
        if i == n {
            let solution = queens
                .iter()
                .map(|row| {
                    (0..n)
                        .map(|i| if row & (1 << i) > 0 { 'Q' } else { '.' })
                        .collect::<String>()
                })
                .collect();
            all.push(solution);
        } else {
            for j in 0..n {
                let column_bit = 1 << j;
                let diagonal_bit = 1 << (i + j);
                let diagona2_bit = 1 << (n + i - j);
                if column_bit & *column == 0
                    && diagonal_bit & *diagonal == 0
                    && diagona2_bit & *diagona2 == 0
                {
                    *column |= column_bit;
                    *diagonal |= diagonal_bit;
                    *diagona2 |= diagona2_bit;
                    queens.push(column_bit);
                    Self::dfs(i + 1, queens, column, diagonal, diagona2, all, n);
                    queens.pop();
                    *column &= !column_bit;
                    *diagonal &= !diagonal_bit;
                    *diagona2 &= !diagona2_bit;
                }
            }
        }
    }
}

#[test]
fn test() {
    let n = 4;
    let res = vec_vec_string![
        [".Q..", "...Q", "Q...", "..Q."],
        ["..Q.", "Q...", "...Q", ".Q.."]
    ];
    assert_eq!(Solution::solve_n_queens(n), res);
}
