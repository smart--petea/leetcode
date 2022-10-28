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
    pub fn search_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        while let Some(root_rc) = root {
            let inner = root_rc.borrow();
            if inner.val == val {
                return Some(root_rc.clone());
            }

            if inner.val > val {
                root = inner.left.clone();
            } else {
                root = inner.right.clone();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node_1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let node_3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let node_2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: node_1, right: node_3})));
        let node_7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let root = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: node_2.clone(), right: node_7})));

        let expected = node_2;
        let output = Solution::search_bst(root.clone(), 2);
        assert_eq!(expected, output);

        let expected = None;
        let output = Solution::search_bst(root.clone(), 5);
        assert_eq!(expected, output);
    }
}
