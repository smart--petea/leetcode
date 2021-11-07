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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let mut now = root.clone();
        while let Some(mut now_rc) = now {
            let mut pre = now_rc.borrow_mut().left.clone();
            if pre.is_some() {
                while pre.as_ref().unwrap().borrow().right.is_some() {
                    pre = pre.unwrap().borrow_mut().right.clone();
                }

                let mut now_refcell = now_rc.borrow_mut();
                pre.unwrap().borrow_mut().right = now_refcell.right.take();
                now_refcell.right = now_refcell.left.take();
            }

            now = now_rc.borrow_mut().right.clone();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flatten() {
        /*
        let mut root = None;
        Solution::flatten(&mut root);

        let mut n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        Solution::flatten(&mut n1);
        println!("n1={:?}", n1);

        let mut n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: None})));
        Solution::flatten(&mut n2);
        println!("n2={:?}", n2);

        let mut n1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n3})));
        Solution::flatten(&mut n2);
        println!("n2={:?}", n2);
        */

        let mut n6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let mut n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: n6})));
        let mut n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let mut n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n3, right: n4})));
        let mut n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n5})));
        Solution::flatten(&mut n1);
        println!("n1={:?}", n1);
    }
}
