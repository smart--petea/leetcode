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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();

        if root.is_none() {
            return result;
        }

        let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = Vec::new();
        let mut current = (root, 0);

        while !stack.is_empty() || current.0.is_some() {
            while let (Some(current_rc), right) = current {
                let current_node = current_rc.borrow();
                stack.push((current_rc.clone(), right));

                current = if current_node.right.is_some() {
                     (current_node.left.clone(), 1)
                } else {
                    (current_node.left.clone(), right + 1)
                };

                result.push('(');
                result.push_str(&current_node.val.to_string());

                if current_node.right.is_some() && current_node.left.is_none() {
                    result.push_str("()");
                } else if current_node.right.is_none() && current_node.left.is_none() {
                    for _ in 0..right {
                        result.push(')');
                    }
                }
            }

            if let Some((stack_rc, right)) = stack.pop() {
                let stack_node = stack_rc.borrow();
                current = (stack_node.right.clone(), right + 1);
            }
        }

        result.chars().skip(1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let expected = "1".to_string();
        let output = Solution::tree2str(n1);
        assert_eq!(expected, output);

        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n3})));
        let expected = "2(1)(3)".to_string();
        let output = Solution::tree2str(n2);
        assert_eq!(expected, output);

        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let n6 = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: None})));
        let n7 = Some(Rc::new(RefCell::new(TreeNode{val: 7, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n4, right: n5})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: n6, right: n7})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n3})));
        let expected = "2(1(4)(5))(3(6)(7))".to_string();
        let output = Solution::tree2str(n2);
        assert_eq!(expected, output);

        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n4, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let expected = "1(2(4))(3)";
        let output = Solution::tree2str(n1);
        assert_eq!(expected, output);

        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n4})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let expected = "1(2()(4))(3)";
        let output = Solution::tree2str(n1);
        assert_eq!(expected, output);
    }
}
