// Definition for a binary tree node.
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
struct Solution{}
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Solution::traverse(root, &mut result);

        result
    }

    pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<i32>) {
        if let Some(rc) = root {
            acc.push(rc.borrow().val);

            Solution::traverse(rc.borrow().left.clone(), acc);
            Solution::traverse(rc.borrow().right.clone(), acc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preorder_traversal() {
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = vec![1];
        assert_eq!(Solution::preorder_traversal(one), result);

        /*
         *
         *                      1
         *                     / \ 
         *                    2   3 
         *                   / \
         *                  4   5
         *
         *
         */
        let five = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let four = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let two = Some(Rc::new(RefCell::new(TreeNode{val:2, left: four, right: five})));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let one = Some(Rc::new(RefCell::new(TreeNode{val:1, left: two, right: three})));
        let result = vec![1, 2, 4, 5, 3];
        assert_eq!(Solution::preorder_traversal(one), result);
    }
}
