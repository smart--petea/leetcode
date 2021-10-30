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
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::inner_build_tree(&inorder[..], &postorder[..])
    }

    fn inner_build_tree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let postorder_len = postorder.len();
        if postorder_len == 0 {
            return None;
        }

        let last_postorder_val = postorder[postorder_len-1];
        let root = Some(Rc::new(RefCell::new(TreeNode::new(last_postorder_val))));
        let inorder_index = inorder.iter().position(|&x| x == last_postorder_val).unwrap();

        root.as_ref().unwrap().borrow_mut().left = Solution::inner_build_tree(&inorder[..inorder_index], &postorder[..inorder_index]);
        root.as_ref().unwrap().borrow_mut().right = Solution::inner_build_tree(&inorder[inorder_index+1..], &postorder[inorder_index..(postorder_len-1)]);

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_tree() {
        let inorder = vec![];
        let postorder = vec![];
        let result = None;
        assert_eq!(Solution::build_tree(inorder, postorder), result);
        let inorder = vec![9,3,15,20,7];
        let postorder = vec![9,15,7,20,3];

        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        assert_eq!(Solution::build_tree(inorder, postorder), n3);
    }
}
