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

struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();

        let mut cur = root;
        while cur.is_some() {
            stack.push(cur.clone());

            cur = cur.unwrap().borrow().left.clone();
        }

        BSTIterator{stack: stack}
    }

    fn next(&mut self) -> i32 {
        let mut cur = self.stack.pop().unwrap();
        let val = cur.as_ref().unwrap().borrow().val;

        if cur.as_ref().unwrap().borrow().right.is_some() {
            cur = cur.unwrap().borrow().right.clone();
            self.stack.push(cur.clone());
            while cur.as_ref().unwrap().borrow().left.is_some() { 
                cur = cur.unwrap().borrow().left.clone();
                self.stack.push(cur.clone());
            }
        }

        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/**
 * Your BSTIterator object will be
 instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool =
 obj.has_next();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsiterator() {
        let n20 = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let n15 = Some(Rc::new(RefCell::new(TreeNode{val: 15, left: n9, right: n20})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: n3, right: n15})));
        let mut obj = BSTIterator::new(n7);

        assert_eq!(obj.next(), 3);
        assert_eq!(obj.next(), 7);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 9);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 15);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 20);
        assert_eq!(obj.has_next(), false);
    }
}
