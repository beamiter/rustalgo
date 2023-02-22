struct Solution;
use std::cmp::Ordering::*;

impl Solution {
    fn search(num: Vec<i32>, target: i32) -> bool {
        if num.is_empty() {
            return false;
        }
        let mut l = 0;
        let mut r = num.len() - 1;
        while l < r {
            let m = l + (r - l) / 2;
            if num[m] == target {
                return true;
            }
            match num[m].cmp(&num[r]) {
                Equal => {
                    r -= 1;
                }
                Less => {
                    if target > num[m] && target <= num[r] {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                Greater => {
                    if target >= num[l] && target < num[m] {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
            }
        }
        num[l] == target
    }
}

#[test]
fn test() {
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 0;
    let res = true;
    assert_eq!(Solution::search(nums, target), res);
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 3;
    let res = false;
    assert_eq!(Solution::search(nums, target), res);
    let nums = vec![1, 1];
    let target = 0;
    let res = false;
    assert_eq!(Solution::search(nums, target), res);
    let nums = vec![1, 3, 1, 1];
    let target = 3;
    let res = true;
    assert_eq!(Solution::search(nums, target), res);
}
