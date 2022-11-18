struct Solution;

impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut l: i32 = 0;
        let mut r: i32 = n as i32 - 1;
        while l <= r {
            let m = (l + r) / 2;
            if target == nums[m as usize] {
                return m as i32
            } else if nums[l as usize] <= nums[m as usize] {
                if nums[l as usize] <= target && target <= nums[m as usize] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if nums[m as usize] <= target && target <= nums[r as usize] {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    assert_eq!(Solution::search(nums, target), 4);
}


