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
    pub fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_rc) = root {
            return 1 + Self::height(&root_rc.borrow().left)
        }

        -1
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_rc) = &root {
            let h = Self::height(&root);

            if h-1 == Self::height(&root_rc.borrow().right) {
                return (1 << h) + Self::count_nodes(root_rc.borrow().right.clone());
            } else {
                return (1 << (h-1)) + Self::count_nodes(root_rc.borrow().left.clone());
            }
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_nodes() {
        let root = None;
        let result = 0;
        assert_eq!(Solution::count_nodes(root), result);

        let n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = 1;
        assert_eq!(Solution::count_nodes(n1), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: None})));
        let result = 2;
        assert_eq!(Solution::count_nodes(n1), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let result = 3;
        assert_eq!(Solution::count_nodes(n1), result);

        let n6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n6, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n4, right: n5})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let result = 6;
        assert_eq!(Solution::count_nodes(n1), result);
    }
}
