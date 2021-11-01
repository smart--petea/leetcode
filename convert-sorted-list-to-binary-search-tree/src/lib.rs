#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }

        fn to_bst(head: &Option<Box<ListNode>>, tail: &Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
            if head == tail {
                return None;
            }

            let mut slow = head;
            let mut fast = head;
            while fast != tail && &fast.as_ref().unwrap().next != tail {
                fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
                slow = &slow.as_ref().unwrap().next;
            }

            let right = to_bst(&slow.as_ref().unwrap().next, &tail);
            let left = to_bst(&head, &slow);
            let thead = TreeNode{
                val: slow.as_ref().unwrap().val,
                right: right,
                left: left,
            };

            Some(Rc::new(RefCell::new(thead)))
        }

        to_bst(&head, &None)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_list_to_bst() {
        let head = None;
        let result = None;
        assert_eq!(Solution::sorted_list_to_bst(head), result);

        let l9 = Some(Box::new(ListNode{val: 9, next: None}));
        let l5 = Some(Box::new(ListNode{val: 5, next: l9}));
        let l_0 = Some(Box::new(ListNode{val: 0, next: l5}));
        let l_3 = Some(Box::new(ListNode{val: -3, next: l_0}));
        let l_10 = Some(Box::new(ListNode{val: -10, next: l_3}));

        let n_10 = Some(Rc::new(RefCell::new(TreeNode::new(-10))));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: -3, left: n_10, right: None})));
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n9 = Some(Rc::new(RefCell::new(TreeNode{val: 9, left: n5, right: None})));
        let n0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: n_3, right: n9})));
        assert_eq!(Solution::sorted_list_to_bst(l_10), n0);

        /*
        let l_3 = Some(Box::new(ListNode{val: -3, next: None}));
        let l_10 = Some(Box::new(ListNode{val: -10, next: l_3}));

        let n_10 = Some(Rc::new(RefCell::new(TreeNode::new(-10))));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: -3, left: n_10, right: None})));
        assert_eq!(Solution::sorted_list_to_bst(l_10), n_3);
        */

        /*
        let l_0 = Some(Box::new(ListNode{val: 0, next: None}));
        let l_3 = Some(Box::new(ListNode{val: -3, next: l_0}));
        let l_10 = Some(Box::new(ListNode{val: -10, next: l_3}));

        let n_10 = Some(Rc::new(RefCell::new(TreeNode::new(-10))));
        let n0 = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: -3, left: n_10, right: n0})));
        assert_eq!(Solution::sorted_list_to_bst(l_10), n_3);
        */

        /*
        let l5 = Some(Box::new(ListNode{val: 5, next: None}));
        let l_0 = Some(Box::new(ListNode{val: 0, next: l5}));
        let l_3 = Some(Box::new(ListNode{val: -3, next: l_0}));
        let l_10 = Some(Box::new(ListNode{val: -10, next: l_3}));

        let n_10 = Some(Rc::new(RefCell::new(TreeNode::new(-10))));
        let n5 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let n_3 = Some(Rc::new(RefCell::new(TreeNode{val: -3, left: n_10, right: None})));
        let n0 = Some(Rc::new(RefCell::new(TreeNode{val: 0, left: n_3, right: n5})));
        assert_eq!(Solution::sorted_list_to_bst(l_10), n0);
        */
    }
}
