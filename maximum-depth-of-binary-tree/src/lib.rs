#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let left_depth = Solution::max_depth(root.as_ref().unwrap().borrow().left.clone());
        let right_depth = Solution::max_depth(root.as_ref().unwrap().borrow().right.clone());
        1 + std::cmp::max(left_depth, right_depth)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_depth() {
        let root = None;
        let result = 0;
        assert_eq!(Solution::max_depth(root), result);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = 1;
        assert_eq!(Solution::max_depth(root), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2})));
        let result = 2;
        assert_eq!(Solution::max_depth(n1), result);

        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 10, left: n15, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        let result = 3;
        assert_eq!(Solution::max_depth(n3), result);
    }
}
