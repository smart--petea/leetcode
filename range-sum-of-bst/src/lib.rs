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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0_i32;

        let mut stack = vec![root];
        while let Some(Some(rc)) = stack.pop() {
            let inner = (*rc).borrow();
            if inner.val >= low && inner.val <= high {
                sum += inner.val;
            }

            if let Some(ref left_rc) = inner.left {
                stack.push(Some(left_rc.clone()));
            }

            if let Some(ref right_rc) = inner.right {
                stack.push(Some(right_rc.clone()));
            }
        }
        
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let three = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let seven = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let eighteen = Some(Rc::new(RefCell::new(TreeNode{val: 18, left: None, right: None})));
        let five = Some(Rc::new(RefCell::new(TreeNode{val: 18, left: three, right: seven})));
        let fifteen = Some(Rc::new(RefCell::new(TreeNode{val: 15, left: None, right: eighteen})));
        let root = Some(Rc::new(RefCell::new(TreeNode{val: 10, left: five, right: fifteen})));
        let low = 7;
        let high = 15;
        let output = Solution::range_sum_bst(root, low, high);
        let expected = 32;
        assert_eq!(output, expected);
    }
}
