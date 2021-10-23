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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root == None {
            return true;
        }

        let mut stack = Vec::new();
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
        let mut root: Option<Rc<RefCell<TreeNode>>>  = root;
        while root != None || !stack.is_empty() {
            while root != None {
                stack.push(root.clone());
                root = root.unwrap().borrow().left.clone();
            }
            root = stack.pop().unwrap();
            if pre != None && root.clone().unwrap().borrow().val <= pre.unwrap().borrow().val {
                return false;
            }

            pre = root.clone();
            root = root.unwrap().borrow().right.clone();
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_bst() {
        /*
         *     2
         *    / \
         *   1   3
         */
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let two = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: three})));
        let result = true;
        assert_eq!(Solution::is_valid_bst(two),  result);

        /*
         *     5
         *    / \
         *   1   4
         *      / \
         *     3   6
         */
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let six = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let four = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: three, right: six})));
        let five = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: one, right: four})));
        let result = false;
        assert_eq!(Solution::is_valid_bst(five),  result);
    }
}
