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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut answer: Vec<String> = Vec::new();
        if root.is_some() {
            Self::search_bt(&root, String::new(), &mut answer);
        }

        answer
    }

    pub fn search_bt(root: &Option<Rc<RefCell<TreeNode>>>, path: String, answer: &mut Vec<String>){
        if let Some(root_rc) = root {
            let root_node = root_rc.borrow();
            if root_node.left.is_none() && root_node.right.is_none() {
                let path_updated = path.clone();
                answer.push(format!("{}{}", path, root_node.val));
            }

            if root_node.left.is_some() {
                Self::search_bt(&root_node.left, format!("{}{}->", path, root_node.val), answer);
            }

            if root_node.right.is_some() {
                Self::search_bt(&root_node.right, format!("{}{}->", path, root_node.val), answer);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree_paths() {
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n5})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let result = vec!["1->2->5", "1->3"];
        assert_eq!(Solution::binary_tree_paths(n1), result);

        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
        let result = vec!["1"];
        assert_eq!(Solution::binary_tree_paths(n1), result);
    }
}
