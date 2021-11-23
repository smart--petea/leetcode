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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            if root_node.left.is_none() && root_node.right.is_none() {
                return 0;
            }

            let mut res = 0i32;
            let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![root_rc.clone()];

            while let Some(curr_rc) = queue.pop() {
                let curr_node = curr_rc.borrow();
                if let Some(curr_left_rc) = &curr_node.left {
                    let curr_left_node = curr_left_rc.borrow();
                    if curr_left_node.left.is_none() && curr_left_node.right.is_none() {
                        res = res + curr_left_node.val;
                    }
                }

                if let Some(curr_left_rc) = &curr_node.left {
                    queue.push(curr_left_rc.clone());
                }

                if let Some(curr_right_rc) = &curr_node.right {
                    queue.push(curr_right_rc.clone());
                }
            }

            return res;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_left_leaves() {
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        let result = 24;
        assert_eq!(Solution::sum_of_left_leaves(n3), result);
    }
}
