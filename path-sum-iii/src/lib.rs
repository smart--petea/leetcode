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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(root_rc) = root.clone() {
            let root_node = root_rc.borrow();
            return Self::path_sum_from(root, sum) + Self::path_sum(root_node.left.clone(), sum) + Self::path_sum(root_node.right.clone(), sum);
        }
        0
    }

    fn path_sum_from(curr: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if let Some(curr_rc) = curr {
            let curr_node = curr_rc.borrow();

            let curr_res = if curr_node.val == sum {
                1
            } else {
                0
            };

            return curr_res + Self::path_sum_from(curr_node.left.clone(), sum-curr_node.val) + Self::path_sum_from(curr_node.right.clone(), sum-curr_node.val);
        }

        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_sum() {
        let n6 = Some(Rc::new(RefCell::new(TreeNode::new(11))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n8 = Some(Rc::new(RefCell::new(TreeNode::new(-2))));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n9})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n7, right: n8})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: -3, left: None, right: n6})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n4, right: n5})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 10, left: n2, right: n3})));
        let target_sum = 8;
        let result = 3;
        assert_eq!(Solution::path_sum(n1, target_sum), result);
    }
}
