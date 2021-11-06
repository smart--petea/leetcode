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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result_list = vec![];
        if root.is_none() {
            return result_list;
        }
        let mut path = vec![];

        Self::path_sum_inner(root, target_sum, &mut path, &mut result_list);

        result_list
    }

    fn path_sum_inner(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, path: &mut Vec<i32>, result_list: &mut Vec<Vec<i32>>) {
        if root.is_none() {
            return;
        }

        let rc = root.unwrap();
        let val = rc.borrow().val;
        let target_sum = target_sum - val;

        path.push(val);

        if target_sum == 0 && rc.borrow().left.is_none() && rc.borrow().right.is_none() {
            result_list.push(path.to_vec());
        } else {
            Self::path_sum_inner(rc.borrow().left.clone(), target_sum, path, result_list);
            Self::path_sum_inner(rc.borrow().right.clone(), target_sum, path, result_list);
        }

        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_sum() {
        let root = None;
        let target_sum = 0;
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root, target_sum), result);

        let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let target_sum = 5;
        let result: Vec<Vec<i32>> = vec![vec![5]];
        assert_eq!(Solution::path_sum(root, target_sum), result);

        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n5, right: n6})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n3, right: n4})));
        let n0 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n2})));
        let target_sum = 5;
        let result: Vec<Vec<i32>> = vec![vec![2, 1, 2], vec![2, 1, 1, 1], vec![2, 3]];
        assert_eq!(Solution::path_sum(n0, target_sum), result);
    }
}
