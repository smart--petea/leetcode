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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

       let (_, _, matches) =  Solution::sum_nodes_matches(root.unwrap());

       matches
    }

    fn sum_nodes_matches(root_rc: Rc<RefCell<TreeNode>>) -> (i32, i32, i32) {
        let root_inner = root_rc.borrow();

        let (sum_left, nodes_left, matches_left) = match root_inner.left {
            Some(ref left_rc) => Solution::sum_nodes_matches(left_rc.clone()),
            _ => (0, 0, 0),
        };

        let (sum_right, nodes_right, matches_right) = match root_inner.right {
            Some(ref right_rc) => Solution::sum_nodes_matches(right_rc.clone()),
            _ => (0, 0, 0),
        };

        let mut matches = matches_left + matches_right;
        let sum = sum_left + sum_right + root_inner.val;
        let nodes = nodes_left + nodes_right + 1;
        if root_inner.val ==  sum / nodes {
            matches += 1;
        }

        (sum, nodes, matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: None})));
        let n8 = Some(Rc::new(RefCell::new(TreeNode{val: 8, left: n0, right: n1})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: n6})));
        let root = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n8, right: n5})));
        let expected = 5;
        let output = Solution::average_of_subtree(root);
        assert_eq!(output, expected);

        let root = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let expected = 1;
        let output = Solution::average_of_subtree(root);
        assert_eq!(output, expected);
    }
}
