struct Solution;

impl Solution {
    fn reverse(x: i32) -> i32 {
        let x_str = x.abs().to_string().chars().rev().collect::<String>();
        if let Ok(y) = x_str.parse::<i32>() {
            x.signum() * y
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let x = 2_147_483_647;
    let res = 0;
    assert_eq!(Solution::reverse(x), res);
}
