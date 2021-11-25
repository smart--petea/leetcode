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
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur: Option<Rc<RefCell<TreeNode>>> = root.clone();
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;

        while let Some(cur_rc) = cur.clone() {
            let cur_node = cur_rc.borrow();

            if cur_node.val == key {
                break;
            }

            pre = Some(cur_rc.clone());

            if key < cur_node.val {
                cur = cur_node.left.clone();
            } else if key > cur_node.val {
                cur = cur_node.right.clone();
            }
        }

        if let Some(pre_rc) = pre {
            let mut pre_node = pre_rc.borrow_mut();
            if pre_node.left == cur {
                pre_node.left = Self::delete_root_node(cur);
            } else {
                pre_node.right = Self::delete_root_node(cur);
            }
        } else {
            return Self::delete_root_node(cur);
        }

        root
    }

    pub fn delete_root_node(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            if root_node.left.is_none() {
                return root_node.right.clone();
            }

            if root_node.right.is_none() {
                return root_node.left.clone();
            }

            let mut next: Option<Rc<RefCell<TreeNode>>> = root_node.right.clone();
            let mut pre: Option<Rc<RefCell<TreeNode>>> = None;
            loop {
                let next_rc = next.clone().unwrap();
                let next_node = next_rc.borrow();
                if next_node.left.is_none() {
                    break;
                }

                pre = Some(next_rc.clone());
                next = next_node.left.clone();
            }

            next.as_ref().unwrap().borrow_mut().left = root_node.left.clone();
            if root_node.right != next.clone() {
                pre.unwrap().borrow_mut().left = next.as_ref().unwrap().borrow().right.clone();
                next.as_ref().unwrap().borrow_mut().right = root_node.right.clone();
            }

            return next;
        }

        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_node() {
        /*
        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let key = 7;
        let result = None;
        assert_eq!(Solution::delete_node(n7, key), result);

        let n6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: n6, right: None})));
        let key = 7;
        let r6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        assert_eq!(Solution::delete_node(n7, key), r6);

        let n6 = Some(Rc::new(RefCell::new(TreeNode::new(6))));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: n6, right: None})));
        let key = 6;
        let r7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(Solution::delete_node(n7, key), r7);
        */

        let n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n2, right: n4})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n3, right: None})));

        let r2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let r4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: r2, right: None})));
        let r5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: r4, right: None})));
        let key = 3;
        assert_eq!(Solution::delete_node(n5, key), r5);

        let n7 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        let n4 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n2, right: n4})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n3, right: n6})));

        let r7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let r2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let r6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: r7})));
        let r4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: r2, right: None})));
        let r5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: r4, right: r6})));
        let key = 3;
        assert_eq!(Solution::delete_node(n5, key), r5);
    }
}
