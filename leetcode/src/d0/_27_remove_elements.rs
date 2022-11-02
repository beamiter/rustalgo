struct Solution;

impl Solution {
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n: usize = nums.len();
        let mut size: usize = 0;
        for i in 0..n {
            if nums[i] != val {
                nums[size] = nums[i];
                size += 1;
            }
        }
        size as i32
    }
}

#[test]
fn test() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    assert_eq!(Solution::remove_element(&mut nums, val), 2);
}
