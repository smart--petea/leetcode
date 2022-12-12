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
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root_rc) = root {
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut current = Some(root_rc.clone());
            let mut sum = 0;

            while stack.len() > 0 || current.is_some() {
                while let Some(current_rc)  = current {
                    let current_node = current_rc.borrow();
                    stack.push(current_rc.clone());
                    current = current_node.right.clone();
                }

                if let Some(stack_rc) = stack.pop() {
                    let mut stack_node = stack_rc.borrow_mut();
                    sum += stack_node.val;
                    stack_node.val = sum;

                    current = stack_node.left.clone();
                }
            }
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n_0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: None, right: None})));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n_8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: None, right: None})));
        let n_5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let n_2 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n_3})));
        let n_1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n_0, right: n_2})));
        let n_7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: n_8})));
        let n_6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: n_5, right: n_7})));
        let n_4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n_1, right: n_6})));

        let output = Solution::bst_to_gst(n_4);
        let output_rc = output.unwrap();
        let output_inner = output_rc.borrow();
        assert_eq!(output_inner.val, 30);
    }
}
