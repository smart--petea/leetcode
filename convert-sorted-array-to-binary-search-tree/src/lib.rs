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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        Solution::helper(&nums, 0i32, (nums.len()-1) as i32)
    }

    fn helper(nums: &Vec<i32>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if low > high {
            return None;
        }

        let mid = ((low + high) as f64 / 2 as f64).ceil() as i32;
        let left = Solution::helper(nums, low, mid-1);
        let right = Solution::helper(nums, mid+1, high);
        
        Some(Rc::new(RefCell::new(TreeNode{val: nums[mid as usize], left: left, right: right})))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_array_to_bst() {
        let nums = vec![];
        let result = None;
        assert_eq!(Solution::sorted_array_to_bst(nums), result);

        let nums = vec![-10, -3, 0, 5, 9];
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: n5, right: None})));
        let n_10 = Some(Rc::new(RefCell::new(TreeNode::new(-10))));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: -3, left: n_10, right: None})));
        let n0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: n_3, right: n9})));
        assert_eq!(Solution::sorted_array_to_bst(nums), n0);

        let nums = vec![1, 3];
        let n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n1, right: None})));
        assert_eq!(Solution::sorted_array_to_bst(nums), n3);
    }
}
