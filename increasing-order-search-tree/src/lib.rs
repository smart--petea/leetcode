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

pub struct Solution;
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }

        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;
        let mut head_rc: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode{val: 0, left: None, right: None}));
        let first_head_rc = head_rc.clone();

        while !stack.is_empty() || current.is_some() {
            while let Some(current_rc)  = current {
                stack.push(current_rc.clone());
                current = (*current_rc).borrow_mut().left.take();
            }

            if let Some(stack_rc) = stack.pop() {
                let stack_node = (*stack_rc).borrow();
                current = stack_node.right.clone();

                let head_rc_clone = head_rc.clone();
                let mut head_tree_node = head_rc_clone.borrow_mut();
                head_tree_node.left = None;
                head_tree_node.right = Some(stack_rc.clone());

                head_rc = stack_rc.clone();
            }
        }

        let right_ref_cell = (*first_head_rc).borrow();
        right_ref_cell.right.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n9 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: None})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n2, right: n4})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n7, right: n9})));
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: n8})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n3, right: n6})));

        let e9 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: None, right: None})));
        let e8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: None, right: e9})));
        let e7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: e8})));
        let e6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: e7})));
        let e5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: e6})));
        let e4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: e5})));
        let e3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: e4})));
        let e2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: e3})));
        let e1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: e2})));

        let output = Solution::increasing_bst(n5);
        assert_eq!(output, e1);

        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n1, right: n7})));

        let e7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let e5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: e7})));
        let e1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: e5})));

        let output = Solution::increasing_bst(n5);
        assert_eq!(output, e1);

        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n3, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n4})));

        let e4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let e3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: e4})));
        let e2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: e3})));
        let e1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: e2})));

        let output = Solution::increasing_bst(n2);
        assert_eq!(output, e1);
    }
}
