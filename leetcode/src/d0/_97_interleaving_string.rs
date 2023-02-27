struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        let n3 = s3.len();
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let s3: Vec<char> = s3.chars().collect();
        return Self::test(&s1, &s2, &s3, n1, n2, n3);
        // let mut memo: HashMap<(usize, usize, usize), bool> = HashMap::new();
        // Self::dp(0, 0, 0, &mut memo, &s1, &s2, &s3, n1, n2, n3)
    }

    fn test(s1: &[char], s2: &[char], s3: &[char], n1: usize, n2: usize, n3: usize) -> bool {
        if n1 + n2 != n3 {
            return false;
        }
        if n1 + n2 == n3 && n3 == 0 {
            return true;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; n1 + 1]; n2 + 1];
        dp[0][0] = 0;
        for i in 0..n2 + 1 {
            for j in 0..n1 + 1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let left = if j >= 1 { dp[i][j - 1] } else { -1 };
                let up = if i >= 1 { dp[i - 1][j] } else { -1 };
                if left >= 0 && s3[left as usize] == s1[j - 1] {
                    dp[i][j] = left + 1
                }
                if up >= 0 && s3[up as usize] == s2[i - 1] {
                    dp[i][j] = up + 1;
                }
            }
        }
        return dp[n2][n1] == n3 as i32;
    }

    fn dp(
        i: usize,
        j: usize,
        k: usize,
        memo: &mut HashMap<(usize, usize, usize), bool>,
        s1: &[char],
        s2: &[char],
        s3: &[char],
        n1: usize,
        n2: usize,
        n3: usize,
    ) -> bool {
        if i == n1 || j == n2 || k == n3 {
            true
        } else {
            if let Some(&res) = memo.get(&(i, j, k)) {
                return res;
            }
            let res = (i < n1
                && k < n3
                && s1[i] == s3[k]
                && Self::dp(i + 1, j, k + 1, memo, s1, s2, s3, n1, n2, n3))
                || (j < n2
                    && k < n3
                    && s2[j] == s3[k]
                    && Self::dp(i, j + 1, k + 1, memo, s1, s2, s3, n1, n2, n3));
            memo.insert((i, j, k), res);
            res
        }
    }
}

#[test]
fn test() {
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbcbcac".to_string();
    let res = true;
    assert_eq!(Solution::is_interleave(s1, s2, s3), res);
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbbaccc".to_string();
    let res = false;
    assert_eq!(Solution::is_interleave(s1, s2, s3), res);
}
