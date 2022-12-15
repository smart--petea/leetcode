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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
       Self::construct_maximum_binary_tree_slice(&nums[..]) 
    }

    pub fn construct_maximum_binary_tree_slice(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        let index = nums
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(i, _)| i)
            .unwrap()
        ;

        let left = Self::construct_maximum_binary_tree_slice(&nums[..index]);
        let right = if index >= nums.len() {
            None
        } else {
            Self::construct_maximum_binary_tree_slice(&nums[index+1..])
        };

        Some(Rc::new(RefCell::new(TreeNode{val: nums[index], left: left, right: right})))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![3, 2, 1, 6, 0, 5];
        let n_1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n_0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: None, right: None})));
        let n_2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n_1})));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n_2})));
        let n_5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n_0, right: None})));
        let n_6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: n_3, right: n_5})));

        let output = Solution::construct_maximum_binary_tree(nums);
        let expected = n_6;
        assert_eq!(output, expected);
    }
}
