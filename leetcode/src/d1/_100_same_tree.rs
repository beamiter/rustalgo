struct Solution;
use std::borrow::Borrow;

use rustgym_util::*;

impl Solution {
    fn is_same_tree(p: TreeLink, q: TreeLink) -> bool {
        if p == None && q == None {
            return true;
        } else if p == None || q == None {
            return false;
        }
        if let Some(node) = p {
        }
        true
    }
}
