struct Solution;

use rustgym_util::*;

impl Solution {
    fn reverse_list(head: ListLink) -> ListLink {
	let mut p = head;
	let mut prev = None;
	while let Some(mut node) = p {
	    p = node.next;
	    node.next = prev;
	    prev = Some(node);
	}
	prev
    }
    fn test(head: ListLink) -> ListLink {
	let mut cur = head;
	let mut pre = None;
	while let Some(mut node) = cur {
	    let temp = node.next;
	    node.next = pre;
	    pre = Some(node);
	    cur = temp;
	}
	pre
    }
}

#[test]
fn test() {
    let head = list!();
    let res = list!();
    assert_eq!(Solution::reverse_list(head), res);
    let head = list!(1);
    let res = list!(1);
    assert_eq!(Solution::reverse_list(head), res);
    let head = list!(1, 2);
    let res = list!(2, 1);
    assert_eq!(Solution::reverse_list(head), res);
    let head = list!(1, 2, 3);
    let res = list!(3, 2, 1);
    assert_eq!(Solution::test(head), res);
}
