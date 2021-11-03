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
    fn min_intern(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rc) = root {
            let left = Self::min_intern(&rc.borrow().left);
            let right = Self::min_intern(&rc.borrow().right);

            if left == 0 && right == 0 {
                return 1;
            }

            if left == 0 {
                return right + 1;
            }

            if right == 0 {
                return left + 1;
            }

            return std::cmp::min(left, right) + 1;
        }

        return 0;
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_intern(&root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_depth() {
        let root = None;
        let result = 0;
        assert_eq!(Solution::min_depth(root), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let result = 1;
        assert_eq!(Solution::min_depth(n2), result);

        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n3})));
        let result = 2;
        assert_eq!(Solution::min_depth(n2), result);

        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        let result = 2;
        assert_eq!(Solution::min_depth(n3), result);

        let n6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: n6})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: n5})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n4})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n3})));
        let result = 5;
        assert_eq!(Solution::min_depth(n2), result);
    }
}
