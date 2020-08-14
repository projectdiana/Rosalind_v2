use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;

// #[derive(Clone,Debug)]
// pub struct TreeNode<T, U>
// where U: std::hash::Hash + Eq {
//     value: T,
//     children: Box<HashMap<U, TreeNode<T, U>>>
// }

// impl<T, U> TreeNode<T, U> 
// where U: std::hash::Hash + Eq {
//     pub fn new(val: T) -> TreeNode<T,U> {
//         TreeNode {
//             value: val,
//             children: Box::new(HashMap::new())
//         }
//     }

//     pub fn add_child(&mut self, label: U, child: TreeNode<T,U>) {
//         self.children.insert(label, child);
//     }

//     pub fn is_leaf(&self) -> bool {
//         self.children.is_empty()
//     }

// }

#[derive(Clone,Debug)]
pub struct TreeNode<T, U>
where U: IntoIterator {
    value: Option<T>,
    children: Option<Box<U>>
}

impl<T, U> TreeNode<T, U> 
where U: IntoIterator {
    pub fn new(val: T) -> TreeNode<T,U> {
        TreeNode {
            value: Some(val),
            children: None
        }
    }

    pub fn add_child(&mut self, label: U, child: TreeNode<T,U>) {
        self.children.insert(label, child);
    }

    pub fn is_leaf(&self) -> bool {
        !self.children.is_some()
    }

}


pub struct Tree<T,U> 
where U: IntoIterator  {
    root: TreeNode<T,U>
}

impl<T,U> Tree<T,U> 
where U: IntoIterator {
    pub fn new(base: T) -> Tree<T,U> {
        Tree {
            root: TreeNode::new(base)
        }
    }

    pub fn level_traverse<V,W>(&self, 
}

// pub struct Tree<T,U> 
// where U: std::hash::Hash + Eq {
//     root: TreeNode<T,U>
// }

// impl<T,U> Tree<T,U> 
// where U: std::hash::Hash + Eq {
//     pub fn new(base: T) -> Tree<T,U> {
//         Tree {
//             root: TreeNode::new(base)
//         }
//     }

//     pub fn level_traverse<V,W>(&self, 
// }

