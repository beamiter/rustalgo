struct Solution;

impl Solution {
    fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(i32, i32)> = vec![(0, 0)];
        heights.push(0);
        let n = heights.len();
        let mut res = 0;
        for i in 0..n {
            let x1 = (i + 1) as i32;
            let y1 = heights[i];
            let mut x3 = x1;
            while let Some(&(x2, y2)) = stack.last() {
                if y2 > y1 {
                    stack.pop();
                    res = res.max((x1 - x2) * y2);
                    x3 = x2;
                } else {
                    stack.push((x3, y1));
                    break;
                }
            }
        }
        res
    }
    fn test0(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut left: Vec<i32> = vec![0; n];
        let mut right: Vec<i32> = vec![0; n];

        let mut mono_stack: Vec<i32> = vec![];
        for i in 0..n {
            while let Some(&x) = mono_stack.last() {
                if heights[x as usize] >= heights[i] {
                    mono_stack.pop();
                } else {
                    break;
                }
            }
            left[i] = if let Some(&x) = mono_stack.last() {
                x
            } else {
                -1
            };
            mono_stack.push(i as i32);
        }
        let mut mono_stack: Vec<i32> = vec![];
        for i in (0..n).rev() {
            while let Some(&x) = mono_stack.last() {
                if heights[x as usize] >= heights[i] {
                    mono_stack.pop();
                } else {
                    break;
                }
            }
            right[i] = if let Some(&x) = mono_stack.last() {
                x
            } else {
                n as i32
            };
            mono_stack.push(i as i32);
        }
        let mut ans: i32 = 0;
        for i in 0..n {
            ans = ans.max((right[i] - left[i] - 1) * heights[i]);
        }
        ans
    }
}

#[test]
fn test() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let res = 10;
    assert_eq!(Solution::test0(heights), res);
}
