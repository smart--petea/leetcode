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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if (root.is_none()) {
            return true;
        }

        let root_rc = root.unwrap();
        let root_inner = root_rc.borrow();
        let val = root_inner.val;
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        if let Some(ref left_rc) = root_inner.left {
            queue.push(left_rc.clone());
        }

        if let Some(ref right_rc) = root_inner.right {
            queue.push(right_rc.clone());
        }

        while let Some(current_rc) = queue.pop() {
            let current_inner = current_rc.borrow();
            if current_inner.val != val {
                return false;
            }

            if let Some(ref left_rc) = current_inner.left {
                queue.push(left_rc.clone());
            }

            if let Some(ref right_rc) = current_inner.right {
                queue.push(right_rc.clone());
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
