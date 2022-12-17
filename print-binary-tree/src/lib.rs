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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = Solution::tree_height(&root) - 1;
        let m = height + 1;
        let n = usize::pow(2, m as u32) - 1;
        let mut result = vec![vec![String::new(); n]; m];

        let r = 0;
        let c = (n-1)/2; 
        if let Some(root_rc) = root {
            Solution::fill_result(&mut result, root_rc, height, r, c);
        }

        result
    }

    fn fill_result(
        result: &mut Vec<Vec<String>>,
        root_rc: Rc<RefCell<TreeNode>>,
        height: usize,
        r: usize,
        c: usize
    ) {
        let root_inner = root_rc.borrow();
        result[r][c] = root_inner.val.to_string();

        if let Some(ref left_rc) = root_inner.left {
            let left_c = c - usize::pow(2, (height-r-1) as u32);
            Solution::fill_result(result, left_rc.clone(), height, r+1, left_c);
        }

        if let Some(ref right_rc) = root_inner.right {
            let right_c = c + usize::pow(2, (height-r-1) as u32);
            Solution::fill_result(result, right_rc.clone(), height, r+1, right_c);
        }
    }

    fn tree_height(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        match root {
            Some(root_rc) => {
                let root_inner = root_rc.borrow();
                let left_height = Solution::tree_height(&root_inner.left);
                let right_height = Solution::tree_height(&root_inner.right);

                1 + usize::max(left_height, right_height)
            }
            None => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: None})));

        let output = Solution::print_tree(n1);
        let expected = vec![vec!["".to_string(), "1".to_string(), "".to_string()], vec!["2".to_string(), "".to_string(), "".to_string()]];
        assert_eq!(output, expected);

        let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));
        let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: None, right: n4})));
        let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
        let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: n2, right: n3})));
        let output = Solution::print_tree(n1);
        let expected = vec![
            vec!["".to_string(), "".to_string(), "".to_string(), "1".to_string(), "".to_string(), "".to_string(), "".to_string()],
            vec!["".to_string(), "2".to_string(), "".to_string(), "".to_string(), "".to_string(), "3".to_string(), "".to_string()],
            vec!["".to_string(), "".to_string(), "4".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        ];
        assert_eq!(output, expected);
    }
}
