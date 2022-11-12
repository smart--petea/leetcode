use std::rc::Rc;
use std::cell::RefCell;

use ost::{TreeNode, Solution};


fn main() {
    let n1 = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
    let n3 = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
    let n4 = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: n3, right: None})));
    let n2 = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: n1, right: n4})));

    let output = Solution::increasing_bst(n2);
    println!("output={:?}", output);
}
