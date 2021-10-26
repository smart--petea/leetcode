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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn traverse(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if p.is_none() && q.is_none() {
                return true;
            }

            if  q.is_none() || p.is_none() {
                return false;
            }

            let p_node = p.as_ref().unwrap().borrow();
            let q_node = q.as_ref().unwrap().borrow();
            if p_node.val != q_node.val {
                return false;
            }

            traverse(p_node.left.clone(), q_node.left.clone()) && traverse(p_node.right.clone(), q_node.right.clone())
        }

        traverse(p, q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_same_tree() {
        let p = None;
        let q = None;
        let result = true;
        assert_eq!(Solution::is_same_tree(p, q), result);

        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let result = true;
        assert_eq!(Solution::is_same_tree(p, q), result);

        let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let result = false;
        assert_eq!(Solution::is_same_tree(p, q), result);
    }
}
