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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root.clone().unwrap().clone()];
        while let Some(rc) = stack.pop() {
            let mut node = rc.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();

            if let Some(left_rc) = left {
                node.right.replace(left_rc.clone());
                stack.push(left_rc);
            }

            if let Some(right_rc) = right {
                node.left.replace(right_rc.clone());
                stack.push(right_rc);
            }
        }
        
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_tree() {
        let root = None;
        let result = None;
        assert_eq!(Solution::invert_tree(root), result);

        let n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let n1_result = n1.clone();
        assert_eq!(Solution::invert_tree(n1), n1_result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: None})));

        let r2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let r1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: r2})));
        assert_eq!(Solution::invert_tree(n1), r1);

        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));

        let r3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let r2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let r1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: r3, right: r2})));
        assert_eq!(Solution::invert_tree(n1), r1);
    }
}
