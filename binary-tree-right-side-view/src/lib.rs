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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Self::right_view(&root, &mut result, 0);
        result
    }

    fn right_view(curr: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>, curr_depth: usize ) {
        if let Some(curr_rc) = curr {
            let curr_refcell = curr_rc.borrow();

            if curr_depth == result.len() {
                result.push(curr_refcell.val)
            }

            Self::right_view(&curr_refcell.right, result, curr_depth + 1);
            Self::right_view(&curr_refcell.left, result, curr_depth + 1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_side_view() {
        let root = None;
        let result = vec![];

        assert_eq!(Solution::right_side_view(root), result);

        let n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n4})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n5})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let result = vec![1, 3, 4];
        assert_eq!(Solution::right_side_view(n1), result);
    }
}
