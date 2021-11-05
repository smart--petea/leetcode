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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(r) = root {
            let new_target_sum = target_sum - r.borrow().val;

            if new_target_sum == 0 && r.borrow().left.is_none() && r.borrow().right.is_none() {
                return true;
            }

            return Self::has_path_sum(r.borrow().left.clone(), new_target_sum) || Self::has_path_sum(r.borrow().right.clone(), new_target_sum)
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_path_sum() {
        let root = None;
        let target_sum = 0;
        let result = false;
        assert_eq!(Solution::has_path_sum(root, target_sum), result);

        let root = None;
        let target_sum = -5;
        let result = false;
        assert_eq!(Solution::has_path_sum(root, target_sum), result);

        let root = None;
        let target_sum = 5;
        let result = false;
        assert_eq!(Solution::has_path_sum(root, target_sum), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n11 = Some(Rc::new(RefCell::new(TreeNode{val: 11, left: n7, right: n2})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n11, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n13 = Some(Rc::new(RefCell::new(TreeNode{val: 13, left: None, right: None})));
        let n4_right = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: n1})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n13, right: n4_right})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n4, right: n8})));
        let target_sum = 22;
        let result = true;
        assert_eq!(Solution::has_path_sum(n5, target_sum), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n11 = Some(Rc::new(RefCell::new(TreeNode{val: 11, left: n7, right: n2})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n11, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n13 = Some(Rc::new(RefCell::new(TreeNode{val: 13, left: None, right: None})));
        let n4_right = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: n1})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n13, right: n4_right})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n4, right: n8})));
        let target_sum = 26;
        let result = true;
        assert_eq!(Solution::has_path_sum(n5, target_sum), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n11 = Some(Rc::new(RefCell::new(TreeNode{val: 11, left: n7, right: n2})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n11, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n13 = Some(Rc::new(RefCell::new(TreeNode{val: 13, left: None, right: None})));
        let n4_right = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: n1})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n13, right: n4_right})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n4, right: n8})));
        let target_sum = 6;
        let result = false;
        assert_eq!(Solution::has_path_sum(n5, target_sum), result);
    }
}
