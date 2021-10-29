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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut result = vec![];
        let mut queue = VecDeque::new();
        let mut to_back = true;

        queue.push_front(root);

        while !queue.is_empty() {
            let mut sub_list = vec![];
            to_back = !to_back;

            for _ in 0..queue.len() {
                let node = queue.pop_back().unwrap();
                let node = node.as_ref().unwrap().borrow();

                if node.left.is_some() {
                    queue.push_front(node.left.clone());
                }

                if node.right.is_some() {
                    queue.push_front(node.right.clone());
                }

                sub_list.push(node.val);
            }

            if to_back {
                sub_list.reverse();
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
    fn zigzag_level_order() {
        let root = None;
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::zigzag_level_order(root), result);

        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        let result: Vec<Vec<i32>> = vec![vec![3], vec![20, 9], vec![15,7]];
        assert_eq!(Solution::zigzag_level_order(n3), result);
    }
}
