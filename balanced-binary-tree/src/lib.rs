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
    fn dfs_height(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if root.is_none() {
            return Some(0);
        }

        let left_height = Self::dfs_height(root.as_ref().unwrap().borrow().left.clone());
        if left_height.is_none() {
            return None;
        }

        let right_height = Self::dfs_height(root.unwrap().borrow().right.clone());
        if right_height.is_none() {
            return None;
        }

        if (right_height.unwrap() - left_height.unwrap()).abs() > 1 {
            return None;
        }

        Some(std::cmp::max(right_height.unwrap(), left_height.unwrap()) + 1)
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        Self::dfs_height(root).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_balanced() {
        let root = None;
        let result = true;
        assert_eq!(Solution::is_balanced(root), result);

        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        let result = true;
        assert_eq!(Solution::is_balanced(n3), result);

        let n4_left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n4_right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n3_left = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n4_left, right: n4_right})));
        let n3_right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2_left = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n3_left, right: n3_right})));
        let n2_right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2_left, right: n2_right})));
        let result = false;
        assert_eq!(Solution::is_balanced(n1), result);
    }
}
