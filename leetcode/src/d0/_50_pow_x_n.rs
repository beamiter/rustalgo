struct Solution;

impl Solution {
    fn my_pow(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    assert_approx_eq!(Solution::my_pow(2.0, 10), 1024.0);
}
