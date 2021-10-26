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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut firstElement: Option<Rc<RefCell<TreeNode>>> = None;
        let mut secondElement: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prevElement: Option<Rc<RefCell<TreeNode>>> = None;

        fn traverse(root: &mut Option<Rc<RefCell<TreeNode>>>, first: &mut Option<Rc<RefCell<TreeNode>>>, second: &mut Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
            if root.is_none() {
                return;
            }

            traverse(&mut root.clone().unwrap().borrow().left.clone(), first, second, prev);

            if prev.is_some() {
                if first.is_none() && prev.as_ref().unwrap().borrow().val >= root.as_ref().unwrap().borrow().val {
                    *first = Some(Rc::clone(prev.as_ref().unwrap()));
                }

                if first.is_some() && prev.as_ref().unwrap().borrow().val >= root.as_ref().unwrap().borrow().val {
                    *second = Some(Rc::clone(root.as_ref().unwrap()));
                }
            }
            *prev = Some(Rc::clone(root.as_ref().unwrap()));

            traverse(&mut root.clone().unwrap().borrow().right.clone(), first, second, prev);

        }

        traverse(root, &mut firstElement, &mut secondElement, &mut prevElement);

        let temp = secondElement.as_ref().unwrap().borrow().val;
        secondElement.as_ref().unwrap().borrow_mut().val = firstElement.as_ref().unwrap().borrow_mut().val;
        firstElement.as_ref().unwrap().borrow_mut().val = temp;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recover_tree() {
        fn show_inorder(root: &Option<Rc<RefCell<TreeNode>>>) {
            if *root == None {
                return;
            }
            show_inorder(&root.as_ref().unwrap().borrow().left.clone());
            println!("val = {}", root.as_ref().unwrap().borrow().val);
            show_inorder(&root.as_ref().unwrap().borrow().right.clone());
        }

        /*
        let mut two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut four = Some(Rc::new(RefCell::new(TreeNode{val: 4i32, left: two, right: None})));
        let mut one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut three = Some(Rc::new(RefCell::new(TreeNode{val: 3i32, left: one, right: four})));
        Solution::recover_tree(&mut three);

        show_inorder(&three);
        */

        /*
        let mut two = Some(Rc::new(RefCell::new(TreeNode::new(2i32))));
        let mut three = Some(Rc::new(RefCell::new(TreeNode{val: 3i32, left: two, right: None})));
        let mut one = Some(Rc::new(RefCell::new(TreeNode{val: 1i32, left: three, right: None})));
        Solution::recover_tree(&mut one);
        */

        /*
        let mut nine = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut three = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let mut min = Some(Rc::new(RefCell::new(TreeNode::new(std::i32::MIN))));
        let mut two = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: three, right: min})));
        let mut five = Some(Rc::new(RefCell::new(TreeNode{val: 5i32, left: two, right: nine})));
        show_inorder(&five);
        println!("-----------");
        Solution::recover_tree(&mut five);
        show_inorder(&five);
        */

        /*
        let mut nine = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let mut two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut min = Some(Rc::new(RefCell::new(TreeNode::new(std::i32::MIN))));
        let mut three = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: min, right: two})));
        let mut five = Some(Rc::new(RefCell::new(TreeNode{val: 5i32, left: three, right: nine})));
        show_inorder(&five);
        println!("-----------");
        Solution::recover_tree(&mut five);
        show_inorder(&five);
        */

        let mut two = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut four = Some(Rc::new(RefCell::new(TreeNode{val: 4i32, left: two.clone(), right: None})));
        let mut one = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let mut three = Some(Rc::new(RefCell::new(TreeNode{val: 3i32, left: one, right: four})));
        show_inorder(&three);
        println!("-----------");
        Solution::recover_tree(&mut three);
        show_inorder(&two);
    }
}
