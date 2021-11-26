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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut cur_option = None;
        let mut cur_count = 0i32;
        let mut max_count = 0i32;
        let mut mode_count = 0i32;
        let mut modes_option: Option<&mut Vec<i32>> = None;
        Self::inorder(&root, &mut cur_option, &mut cur_count, &mut max_count, &mut mode_count, &mut modes_option);

        let mut modes = vec![0i32; mode_count as usize];
        modes_option = Some(&mut modes);
        mode_count = 0;
        cur_count = 0;
        Self::inorder(&root, &mut cur_option, &mut cur_count, &mut max_count, &mut mode_count, &mut modes_option);

        modes
    }

    fn inorder<'a>(root: &Option<Rc<RefCell<TreeNode>>>, cur_option: &mut Option<i32>, cur_count: &'a mut i32, max_count: &'a mut i32, mode_count: &mut i32, modes_option: &mut Option<&mut Vec<i32>>) {
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            Self::inorder(&root_node.left, cur_option, cur_count, max_count, mode_count, modes_option);
            Self::handle_value(root_node.val, cur_option, cur_count, max_count, mode_count, modes_option);
            Self::inorder(&root_node.right, cur_option, cur_count, max_count, mode_count, modes_option);
        }
    }

    fn handle_value<'a>(val: i32, cur_option: &mut Option<i32>, cur_count: &'a mut i32, max_count: &'a mut i32, mode_count: &mut i32, modes_option: &mut Option<&mut Vec<i32>>) {
        if let Some(cur_val) = cur_option {
            if *cur_val != val {
                cur_option.replace(val);
                *cur_count = 0i32;
            }
        } else {
            cur_option.replace(val);
        }

        *cur_count = *cur_count + 1;
        if *cur_count > *max_count {
            *max_count = *cur_count;
            *mode_count = 1i32;
        } else if *cur_count == *max_count {
            if let Some(ref mut modes) = modes_option {
                modes[(*mode_count) as usize] = val;
            }

            *mode_count = *mode_count + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_mode() {
        let n2_0 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n2_1 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n2_0, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2_1})));
        let result = vec![2];
        assert_eq!(Solution::find_mode(n1), result);

        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2})));
        let result = vec![1, 2];
        assert_eq!(Solution::find_mode(n1), result);
    }
}
