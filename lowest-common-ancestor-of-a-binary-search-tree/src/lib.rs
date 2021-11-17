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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let mut root_val: i32;

        loop {
            root_val = root.clone().unwrap().borrow().val;

            if (root_val - p_val) * (root_val - q_val) <= 0 {
                break;
            }

            root = if p_val < root_val {
                root.unwrap().borrow().left.clone()
            } else {
                root.unwrap().borrow().right.clone()
            }
        }

        root
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_ancestor() {
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n3, right: n5})));
        let n0 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n0, right: n4.clone()})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n7, right: n9})));
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: n2.clone(), right: n8})));

        let p = n2.clone();
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let result: Option<Rc<RefCell<TreeNode>>> = n6.clone();
        assert_eq!(Solution::lowest_common_ancestor(n6.clone(), p, q), result);

        let p = n2.clone();
        let q = n4.clone();
        let result = n2.clone();
        assert_eq!(Solution::lowest_common_ancestor(n6.clone(), p, q), result);

        let p = n2.clone();
        let q = n4.clone();
        let result = n2.clone();
        assert_eq!(Solution::lowest_common_ancestor(n6.clone(), p, q), result);
    }
}
