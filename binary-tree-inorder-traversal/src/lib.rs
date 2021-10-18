use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        let mut cur = root;
        while cur != None || !stack.is_empty() {
            while cur != None {
                stack.push(cur.clone());
                cur = match &(*cur.unwrap()).borrow().left {
                    Some(l) => Some(l.clone()),
                    None => None,
                };
            }
            cur = stack.pop().unwrap();
            result.push((*cur.clone().unwrap()).borrow().val);
            cur = match &(*cur.unwrap()).borrow().right {
                Some(r) => Some(r.clone()),
                None => None,
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inorder_traversal() {
        let one: Option<Rc<RefCell<TreeNode>>> = None;
        let result = vec![];
        assert_eq!(Solution::inorder_traversal(one), result);

        /*
         *         1
         */
        let one: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = vec![1];
        assert_eq!(Solution::inorder_traversal(one), result);

        /*
         *         1
         *           \
         *            2
         *
         */
        let two = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let one: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: two})));
        let result = vec![1, 2];
        assert_eq!(Solution::inorder_traversal(one), result);

        /*
         *         2
         *        /   
         *       1     
         *
         */
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let two: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: None})));
        let result = vec![1, 2];
        assert_eq!(Solution::inorder_traversal(two), result);

        /*
         *         2
         *        / \  
         *       1   3  
         *
         */
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let two: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: three})));
        let result = vec![1, 2, 3];
        assert_eq!(Solution::inorder_traversal(two), result);

        /*
         *           4
         *          /
         *         2
         *        / \  
         *       1   3  
         *
         */
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let two: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: three})));
        let four: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: two, right: None})));

        let result = vec![1, 2, 3, 4];
        assert_eq!(Solution::inorder_traversal(four), result);

        /*
         *           2
         *          / \
         *         1   4 
         *            / \ 
         *           3   5
         *                \
         *                 6
         *
         */
        let one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let six = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let five: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: six})));
        let four: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: three, right: five})));
        let two: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: four})));

        let result = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::inorder_traversal(two), result);
    }
}
