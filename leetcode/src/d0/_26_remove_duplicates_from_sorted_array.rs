struct Solution;

impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut last = nums[0];
        let mut size = 1;
        for i in 1..n {
            if nums[i] != last {
                last = nums[i];
                nums[size] = nums[i];
                size += 1;
            }
        }
        nums.resize(size, 0);
        nums.shrink_to_fit();
        size as i32
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
}
