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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        let mut stack = vec![root.unwrap()];
        while let Some(cur_rc) = stack.pop() {
            let mut cur_node = cur_rc.borrow_mut();
            result.push(cur_node.val);

            if cur_node.left.is_some() {
                stack.push(cur_node.left.take().unwrap().clone());
            }

            if cur_node.right.is_some() {
                stack.push(cur_node.right.take().unwrap().clone());
            }
        }

        result.reverse();
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postorder_traversal() {
        let root = None;
        let result = vec![];
        assert_eq!(Solution::postorder_traversal(root), result);

        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n3, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2})));
        let result = vec![3, 2, 1];
        assert_eq!(Solution::postorder_traversal(n1), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: None})));
        let result = vec![2, 1];
        assert_eq!(Solution::postorder_traversal(n1), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2})));
        let result = vec![2, 1];
        assert_eq!(Solution::postorder_traversal(n1), result);
    }
}
