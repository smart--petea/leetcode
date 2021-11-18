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
        let mut ans: Option<Rc<RefCell<TreeNode>>> = None;
        Self::recurse_tree(&root, &p, &q, &mut ans);

        ans
    }

    pub fn recurse_tree(cur_node: &Option<Rc<RefCell<TreeNode>>>, p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if cur_node.is_none() {
            return 0;
        }

        let cur_node_inner = cur_node.as_ref().unwrap().borrow();
        let left = Self::recurse_tree(&cur_node_inner.left, p, q, ans);
        let right = Self::recurse_tree(&cur_node_inner.right, p, q, ans);
        let mid = if cur_node == p || cur_node == q { 1 } else { 0 };

        if mid + left + right >= 2 {
            *ans = Some(cur_node.as_ref().unwrap().clone());
        }

        if mid + left + right > 0 {
            1
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_ancestor() {
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: None, right: None}))); 
        let n0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: None, right: None}))); 
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n0, right: n8}))); 
        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None}))); 
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None}))); 
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n7, right: n4.clone()}))); 
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: None}))); 
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: n6, right: n2}))); 
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n5.clone(), right: n1.clone()}))); 
        let p = n5.clone();
        let q = n1.clone();
        let result = n3.clone();
        assert_eq!(Solution::lowest_common_ancestor(n3.clone(), p, q), result);

        let p = n5.clone();
        let q = n4.clone();
        let result = n5.clone();
        assert_eq!(Solution::lowest_common_ancestor(n5.clone(), p, q), result);

        let p = n5.clone();
        let q = n4.clone();
        let result = n5.clone();
        assert_eq!(Solution::lowest_common_ancestor(n5.clone(), p, q), result);
    }
}
