struct Solution;

impl Solution {
    fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        match nums.binary_search(&target) {
            Ok(i) => {
                let mut l = i;
                let mut r = i;
                while l > 0 && nums[l - 1] == target {
                    l -= 1;
                }
                while r + 1 < n && nums[r + 1] == target {
                    r += 1;
                }
                vec![l as i32, r as i32]
            }
            Err(_) => vec![-1, -1],
        }
    }
}
