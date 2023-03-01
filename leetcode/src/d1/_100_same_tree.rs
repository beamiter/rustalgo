struct Solution;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use rustgym_util::*;

impl Solution {
    fn is_same_tree(p: TreeLink, q: TreeLink) -> bool {
        Solution::is_same_tree_impl(&p, &q)
    }
    fn is_same_tree_impl(p: &TreeLink, q: &TreeLink) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let a: &TreeNode = &p.as_ref().unwrap().borrow_mut();
        let b: &TreeNode = &q.as_ref().unwrap().borrow_mut();
        if a.val != b.val {
            return false;
        }
        Solution::is_same_tree_impl(&a.left, &b.left)
            && Solution::is_same_tree_impl(&a.right, &b.right)
    }
}

#[test]
fn test() {
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let p = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let res = true;
    assert_eq!(Solution::is_same_tree(p, q), res);
}

