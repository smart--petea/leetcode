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
use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut sum = 0;
        let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        q.push_front((root.unwrap(), 0i32));
        while q.is_empty() == false {
            if let Some((n, cur)) = q.pop_back() {
                let n_refcell = n.borrow();
                let cur = cur * 10 + n_refcell.val;

                if n_refcell.left.is_none() && n_refcell.right.is_none() {
                    sum = sum + cur;
                    continue;
                }

                if let Some(left) = &n_refcell.left {
                    q.push_front((left.clone(), cur));
                }

                if let Some(right) = &n_refcell.right {
                    q.push_front((right.clone(), cur));
                }
            }
        }

        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_numbers() {
        let root = None;
        let result = 0;
        assert_eq!(Solution::sum_numbers(root), result);

        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let result = 25;
        assert_eq!(Solution::sum_numbers(n1), result);

        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: n5, right: n1})));
        let n0 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n9, right: n0})));
        let result = 1026;
        assert_eq!(Solution::sum_numbers(n4), result);
    }
}
