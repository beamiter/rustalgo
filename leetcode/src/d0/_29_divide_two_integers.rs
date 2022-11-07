struct Solution;

use std::i32;

impl Solution {
    fn divide(dividend: i32, divisor: i32) -> i32 {
        if let Some(res) = dividend.checked_div(divisor) {
            res
        } else {
            i32::MAX
        }
    }
}

#[test]
fn test() {
    let dividend = 10;
    let divisor = 3;
    let res = 3;
    assert_eq!(Solution::divide(dividend, divisor), res);
}
