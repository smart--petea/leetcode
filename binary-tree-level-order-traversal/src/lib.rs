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

struct Solution ;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        let mut number = 0;
        queue.push_front(root);
        while queue.is_empty() == false {
            number = queue.len();
            let mut sub_list = vec![];
            loop {
                let node = queue.pop_back().unwrap();
                if node.as_ref().unwrap().borrow().left.is_some() {
                    queue.push_front(node.as_ref().unwrap().borrow().left.clone());
                }

                if node.as_ref().unwrap().borrow().right.is_some() {
                    queue.push_front(node.as_ref().unwrap().borrow().right.clone());
                }

                sub_list.push(node.unwrap().borrow().val);

                number = number-1;
                if number == 0 {
                    break;
                }
            }
            result.push(sub_list);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_order() {
        let root = None;
        let result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::level_order(root), result);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result: Vec<Vec<i32>> = vec![vec![1]];
        assert_eq!(Solution::level_order(root), result);

        let seven = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let fifteen = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let twenty = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: fifteen, right: seven})));
        let nine = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let three = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: nine, right: twenty})));
        let result: Vec<Vec<i32>> = vec![vec![3], vec![9, 20], vec![15,7]];
        assert_eq!(Solution::level_order(three), result);
    }
}
