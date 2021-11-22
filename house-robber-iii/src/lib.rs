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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let result = Self::dfs(&root);
        std::cmp::max(result[0], result[1])
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            let left = Self::dfs(&root_node.left);
            let right = Self::dfs(&root_node.right);

            let mut res: [i32; 2] = [0i32; 2];
            res[0] = left[1] + right[1] + root_node.val;
            res[1] = std::cmp::max(left[0], left[1]) + std::cmp::max(right[0], right[1]);

            return res;
        }

        [0i32; 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rob() {
        let n3_3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n3_3})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let n3_2 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n1})));
        let n3_1 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n2, right: n3_2})));
        let result = 7;
        assert_eq!(Solution::rob(n3_1), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let result = 2;
        assert_eq!(Solution::rob(n2), result);
    }
}
