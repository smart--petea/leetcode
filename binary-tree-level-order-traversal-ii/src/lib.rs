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
use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_front(root);
        while queue.is_empty() == false {
            let mut nodes = vec![];
            for _ in 0..queue.len() {
                let node_rc = queue.pop_back().unwrap().unwrap();
                let node = node_rc.borrow();

                nodes.push(node.val);

                if node.left.is_some() {
                    queue.push_front(Some(node.left.as_ref().unwrap().clone()));
                }

                if node.right.is_some() {
                    queue.push_front(Some(node.right.as_ref().unwrap().clone()));
                }
            }
            result.push(nodes);
        }
        result.reverse();

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn level_order_bottom() {
        let root = None;
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order_bottom(root), result);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result: Vec<Vec<i32>> = vec![vec![1]];
        assert_eq!(Solution::level_order_bottom(root), result);

        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        let result: Vec<Vec<i32>> = vec![vec![15,7], vec![9,20], vec![3]];
        assert_eq!(Solution::level_order_bottom(n3), result);
    }
}
