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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root_rc = root.as_ref().unwrap();
        let root_tree_node = root_rc.borrow();
        let root_val = root_tree_node.val;

        let left_val: i32 = match root_tree_node.left.as_ref() {
            Some(ref left_rc) => {
                left_rc.borrow().val
            }
            None => 0
        };

        let right_val: i32 = match root_tree_node.right.as_ref() {
            Some(ref right_rc) => {
                right_rc.borrow().val
            }
            None => 0
        };


        root_val == left_val + right_val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut root = TreeNode::new(10);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let root = Some(Rc::new(RefCell::new(root)));

        let expected = true;
        let output = Solution::check_tree(root);
        assert_eq!(expected, output);
    }
}
