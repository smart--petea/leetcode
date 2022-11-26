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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut averages = Vec::new();
        if root.is_none() {
            return averages;
        }

        let mut current_level: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap()];
        let mut next_level: Vec<Rc<RefCell<TreeNode>>> = Vec::new();

        let mut sum = 0f64;
        while current_level.len() > 0 {
            sum = 0f64;
            for current_rc in current_level.iter() {
                let current_node = current_rc.borrow();
                
                if let Some(ref left_rc) = current_node.left {
                    next_level.push(left_rc.clone());
                }

                if let Some(ref right_rc) = current_node.right {
                    next_level.push(right_rc.clone());
                }

                sum = sum + (current_node.val as f64);
            }

            averages.push(sum / (current_level.len() as f64));
            current_level.clear();
            std::mem::swap(&mut current_level, &mut next_level);
        }

        averages
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n9 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: None, right: None})));
        let n15 = Some(Rc::new(RefCell::new(TreeNode{val: 15, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n20 = Some(Rc::new(RefCell::new(TreeNode{val: 20, left: n15, right: n7})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n9, right: n20})));

        let expected = vec![3f64, 14.5, 11f64];
        let output = Solution::average_of_levels(n3);
        assert_eq!(expected, output);
    }
}
