use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

// Definition for a binary tree node.
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
            right: None,
        }
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node = node.borrow();
            1 + max(max_depth(node.left.clone()), max_depth(node.right.clone()))
        }
        None => 0,
    }
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_valid_bst_with_range(root).0
}

fn is_valid_bst_with_range(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32, i32) {
    match root {
        None => (true, i32::MAX, i32::MIN),
        Some(node) => {
            let node_ref = node.borrow();
            let (left_is_bst, left_min, left_max) = is_valid_bst_with_range(node_ref.left.clone());
            let (right_is_bst, right_min, right_max) =
                is_valid_bst_with_range(node_ref.right.clone());

            let is_bst = left_is_bst
                && right_is_bst
                && (node_ref.left.is_none() || left_max < node_ref.val)
                && (node_ref.right.is_none() || right_min > node_ref.val);

            // Compute min and max values of the subtree
            let min_val = if node_ref.left.is_some() {
                left_min
            } else {
                node_ref.val
            };
            let max_val = if node_ref.right.is_some() {
                right_max
            } else {
                node_ref.val
            };

            (is_bst, min_val, max_val)
        }
    }
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return root == rotate(root.clone());
}

pub fn rotate(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => return None,
        Some(node) => {
            let node_ptr = node.borrow();
            let rotated = TreeNode {
                val: node_ptr.val,
                left: rotate(node_ptr.right.clone()),
                right: rotate(node_ptr.left.clone()),
            };
            return Some(Rc::new(RefCell::new(rotated)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = max_depth(None);
        assert_eq!(result, 0);
    }

    #[test]
    fn one_node_depth() {
        let result = max_depth(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))));
        assert_eq!(result, 2);
    }
}
