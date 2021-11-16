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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count = Vec::new();

        Self::helper(&root, &mut count);

        count[(k-1) as usize]
    }

    fn helper(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let node_inner = node_rc.borrow();
            Self::helper(&node_inner.left, count);
            count.push(node_inner.val);
            Self::helper(&node_inner.right, count);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kth_smallest() {
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n1, right: n4})));
        let k = 1;
        let result = 1;
        assert_eq!(Solution::kth_smallest(n3, k), result);

        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n1, right: n4})));
        let k = 2;
        let result = 2;
        assert_eq!(Solution::kth_smallest(n3, k), result);
    }
}
