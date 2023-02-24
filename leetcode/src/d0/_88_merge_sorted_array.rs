struct Solution;

impl Solution {
    fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);
        nums1.append(nums2);
        nums1.sort_unstable();
    }
    fn test(nums1: &mut Vec<i32>, l: i32, nums2: &mut Vec<i32>, r: i32) {
        let mut m = l;
        let mut n = r;
        for i in (0..(m + n)).rev() {
            if n == 0 {
                break;
            } else {
                if m == 0 {
                    nums1[i as usize] = nums2[n as usize - 1];
                    n -= 1;
                    continue;
                }
                if nums1[m as usize - 1] > nums2[n as usize - 1] {
                    nums1[i as usize] = nums1[m as usize - 1];
                    m -= 1;
                } else {
                    nums1[i as usize] = nums2[n as usize - 1];
                    n -= 1;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    let m = 3;
    let n = 3;
    Solution::test(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
