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
            right: None
        }
    }

    pub fn set_left(node: Option<Rc<RefCell<TreeNode>>>, left: Option<Rc<RefCell<TreeNode>>>) {
        (*node.unwrap()).borrow_mut().left = left;
    }

    pub fn set_right(node: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) {
        (*node.unwrap()).borrow_mut().right = right;
    }
}

use std::rc::Rc;
use std::cell::RefCell;
pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>, 
        p: Option<Rc<RefCell<TreeNode>>>, 
        q: Option<Rc<RefCell<TreeNode>>>) 
        -> Option<Rc<RefCell<TreeNode>>> 
{
    if q.is_none() { return None }
    if p.is_none() { return None }
    if root.is_none() { return None }
    
    let q = q.unwrap();
    let p = p.unwrap();
    let root = root.unwrap();

    // Handle case where the current root between both leafs
    // Base case for recursion
    if (q.borrow().val >= root.borrow().val && 
        p.borrow().val <= root.borrow().val) || 
        (p.borrow().val >= root.borrow().val && 
        q.borrow().val <= root.borrow().val) 
    {
        return Some(root);
    } 

    // Figure out if we should try the left or right leaf next
    if q.borrow().val > root.borrow().val && p.borrow().val > root.borrow().val
    {
        return lowest_common_ancestor(root.borrow().right.clone(), Some(p), Some(q));
    } else {
        return lowest_common_ancestor(root.borrow().left.clone(), Some(p), Some(q));
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root = TreeNode::new(val);
        Some(Rc::new(RefCell::new(root)))
    }

    #[test]
    fn it_works() {
        let node = create_node(4);
        TreeNode::set_left(node.clone(), create_node(3));
        TreeNode::set_right(node.clone(), create_node(5));

        let node1 = create_node(2);
        TreeNode::set_right(node1.clone(), node);
        TreeNode::set_left(node1.clone(), create_node(0));

        let node = create_node(8);
        TreeNode::set_left(node.clone(), create_node(7));
        TreeNode::set_right(node.clone(), create_node(9));

        let root = create_node(6);
        TreeNode::set_left(root.clone(), node1);
        TreeNode::set_right(root.clone(), node);


        let p = create_node(2);
        let q = create_node(4);

        let ans = lowest_common_ancestor(root, p, q);
        assert_eq!((*ans.unwrap()).borrow().val, 2);
    }
}