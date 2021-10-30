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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        if preorder.len() == 1 {
            return root;
        }

        let mid = inorder.iter().position(|&x| x==preorder[0]).unwrap();
        root.as_ref().unwrap().borrow_mut().left = Solution::build_tree(preorder[1..=mid].to_vec(), inorder[..mid].to_vec());
        root.as_ref().unwrap().borrow_mut().right = Solution::build_tree(preorder[mid+1..].to_vec(), inorder[mid+1..].to_vec());

        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_tree() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));
        assert_eq!(Solution::build_tree(preorder, inorder), n3);
    }
}
