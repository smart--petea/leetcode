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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        fn traverse(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if root1.is_none() && root2.is_none() {
                return true;
            }

            if root1.is_none() || root2.is_none() {
                return false;
            }

            let root1_node = root1.as_ref().unwrap().borrow();
            let root2_node = root2.as_ref().unwrap().borrow();

            if root1_node.val != root2_node.val {
                return false;
            }

            traverse(root1_node.left.clone(), root2_node.right.clone()) && traverse(root1_node.right.clone(), root2_node.left.clone())
        }

        let t_node = root.as_ref().unwrap().borrow();
        traverse(t_node.left.clone(), t_node.right.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_symmetric() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        let result = true;
        assert_eq!(Solution::is_symmetric(root), result);

        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let result = true;
        assert_eq!(Solution::is_symmetric(root), result);

        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: left, right: None})));
        let result = false;
        assert_eq!(Solution::is_symmetric(root), result);

        let left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: left, right: right})));
        let result = true;
        assert_eq!(Solution::is_symmetric(root), result);

        let three: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let four: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let two_left: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: three.clone(), right: four.clone()})));
        let two_right: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: four, right: three})));
        let one: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: two_left, right: two_right})));
        let result = true;
        assert_eq!(Solution::is_symmetric(one), result);

        let three: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let two: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: three})));
        let one: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: two.clone(), right: two})));
        let result = false;
        assert_eq!(Solution::is_symmetric(one), result);
    }
}
