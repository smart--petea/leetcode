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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0i32;

        if root.is_none() {
            return sum;
        }

        let mut stack: Vec<(i32, i32, Rc<RefCell<TreeNode>>)> = vec![(1, 1, root.unwrap())]; //[(grandparent, parent, node)]
        while let Some((grandparent, parent, stack_rc)) = stack.pop() {
            let stack_node = stack_rc.borrow();
            if grandparent % 2 == 0 {
                sum += stack_node.val;
            }

            let grandparent = parent;
            let parent = stack_node.val;

            if let Some(ref left_rc) = stack_node.left {
                stack.push((grandparent, parent, left_rc.clone()));
            }

            if let Some(ref right_rc) = stack_node.right {
                stack.push((grandparent, parent, right_rc.clone()));
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
        let n_7 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: None, right: None})));
        let n_8 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n_9 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n_10 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n_7, right: None})));
        let n_4 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: n_8, right: n_9})));
        let n_5 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n_6 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n_10})));
        let n_1 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: n_3, right: n_4})));
        let n_2 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n_5, right: n_6})));
        let root = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: n_1, right: n_2})));

        let expected = 18;
        let output = Solution::sum_even_grandparent(root);
        assert_eq!(expected, output);


        let root = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let expected = 0;
        let output = Solution::sum_even_grandparent(root);
        assert_eq!(expected, output);
    }
}
