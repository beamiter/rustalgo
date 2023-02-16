struct Solution;

impl Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut a = vec![];
        for row in matrix {
            for x in row {
                a.push(x);
            }
        }
        a.binary_search(&target).is_ok()
    }
    fn search_0(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let mut i: i32 = 0;
        let mut j: i32 = matrix[0].len() as i32 - 1;
        while i < matrix.len() as i32 && j >= 0 {
            let i0 = i as usize;
            let j0 = j as usize;
            if matrix[i0][j0] == target {
                return true;
            } else if matrix[i0][j0] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return false;
    }
    fn search_1(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let mut left = 0;
        let mut right = matrix.len();
        while left < right {
            let mid = (left + right) / 2;
            if matrix[mid][0] == target {
                return true;
            }
            if matrix[mid][0] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let tmp = if right > 0 { right - 1 } else { right };
        left = 0;
        right = matrix[tmp].len();
        while left < right {
            let mid = (left + right) / 2;
            if matrix[tmp][mid] == target {
                return true;
            }
            if matrix[tmp][mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return false;
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]];
    let target = 23;
    let res = true;
    assert_eq!(Solution::search_1(matrix, target), res);
}
