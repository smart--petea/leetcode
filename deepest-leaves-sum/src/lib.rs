use std::rc::Rc;
use std::cell::{RefCell, Ref};

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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut sum = 0;
        let mut sum_level = 0;

        let mut stack: Vec<(Rc<RefCell<TreeNode>>, usize)> = vec![(root.unwrap(), 0)];
        
        while let Some((stack_rc, stack_level)) = stack.pop() {
            let stack_inner: Ref<TreeNode> = stack_rc.borrow();

            if let Some(ref left_rc) = stack_inner.left {
                stack.push((left_rc.clone(), stack_level + 1));
            }

            if let Some(ref right_rc) = stack_inner.right {
                stack.push((right_rc.clone(), stack_level + 1));
            }

            if stack_level < sum_level {
                
            } else if stack_level > sum_level {
                sum_level = stack_level;
                sum = stack_inner.val;
            } else {
                sum = sum + stack_inner.val;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: None, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n7, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n4, right: n5})));
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: n8})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n6})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));

        let expected = 15;
        let output = Solution::deepest_leaves_sum(n1);
        assert_eq!(expected, output);

    }
}
