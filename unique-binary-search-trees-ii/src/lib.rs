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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Solution::gen_trees(1, n)
    }

    pub fn gen_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut list = Vec::new();

        if start > end {
            list.push(None);
            return list
        }

        if start == end {
            let root = Some(Rc::new(RefCell::new(TreeNode::new(start))));
            list.push(root);
            return list;
        }

        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        for i in start..=end {
            left_list = Solution::gen_trees(start, i-1);
            right_list = Solution::gen_trees(i+1, end);

            for left in &left_list {
                for right in &right_list {
                    let left = match left {
                        Some(l) => Some(l.clone()),
                        None => None,
                    };

                    let right = match right {
                        Some(r) => Some(r.clone()),
                        None => None,
                    };

                    let root = Some(Rc::new(RefCell::new(TreeNode{val: i, left: left, right: right})));
                    list.push(root);
                }
            }

        }

        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_trees() {
        let n = 1;
        let result = vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))];
        assert_eq!(Solution::generate_trees(n), result);
    }
}
