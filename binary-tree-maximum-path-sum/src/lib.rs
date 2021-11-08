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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut max_sum = std::i32::MIN;
        Self::max_sum_helper(&root, &mut max_sum);

        max_sum
    }

    fn max_sum_helper(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        let mut max_one_node_root = 0;
        let mut max_all = std::i32::MIN;

        if let Some(rc) = root {
            let node = rc.borrow();

            let left_max = Self::max_sum_helper(&node.left, max_sum);
            let right_max = Self::max_sum_helper(&node.right, max_sum);

            let max_right_left = std::cmp::max(left_max, right_max);
            max_one_node_root = std::cmp::max(node.val, node.val + max_right_left);
            max_all = std::cmp::max(max_one_node_root, left_max + right_max + node.val);
        }

        *max_sum = std::cmp::max(*max_sum, max_all);

        max_one_node_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_path_sum() {
        let root = None;
        let result = 0;
        assert_eq!(Solution::max_path_sum(root), result);

        let n_3 = Some(Rc::new(RefCell::new(TreeNode::new(-3))));
        let result = -3;
        assert_eq!(Solution::max_path_sum(n_3), result);

        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let result = 6;
        assert_eq!(Solution::max_path_sum(n1), result);

        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n_10 = Some(Rc::new(RefCell::new(TreeNode{val: -10, left: n9, right: n20})));
        let result = 42;
        assert_eq!(Solution::max_path_sum(n_10), result);
    }
}
