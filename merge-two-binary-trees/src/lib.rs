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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }

        if root2.is_none() {
            return root1;
        }

        let mut stack: Vec<(Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>)> = Vec::new();
        let mut current1 = root1.clone();
        let mut current2 = root2.clone();

        while !stack.is_empty() || (current1.is_some() && current2.is_some()) {
            while current1.is_some() &&  current2.is_some() {
                let rc1 = current1.unwrap();
                let rc2 = current2.unwrap();
                stack.push((rc1.clone(), rc2.clone()));
                current1 = rc1.borrow().left.clone();
                current2 = rc2.borrow().left.clone();
            }

            if let Some((rc1, rc2)) = stack.pop() {
                let mut inner1 = rc1.borrow_mut();
                let inner2 = rc2.borrow();


                inner1.val = inner1.val + inner2.val;
                if inner1.left.is_none() && inner2.left.is_some() {
                    inner1.left = inner2.left.clone();
                }

                if inner1.right.is_some() && inner2.right.is_some() {
                    current1 = inner1.right.clone();
                    current2 = inner2.right.clone();
                }

                if inner1.right.is_none() && inner2.right.is_some() {
                    inner1.right = inner2.right.clone();
                }
            }
        }

        root1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n1_5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let n1_2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n1_3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n1_5, right: None})));
        let root1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n1_3, right: n1_2})));

        let n2_4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n2_1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: n2_4})));
        let n2_7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n2_3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: n2_7})));
        let root2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n2_1, right: n2_3})));

        let e_5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let e1_4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let e2_4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: e_5, right: e1_4})));
        let e_7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let e_5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: e_7})));
        let expected_root = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: e2_4, right: e_5})));

        let output = Solution::merge_trees(root1, root2);
        assert_eq!(output, expected_root);
    }
}
