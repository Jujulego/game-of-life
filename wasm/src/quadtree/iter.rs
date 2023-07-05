use std::iter::FusedIterator;
use na::Point2;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

pub struct Iter<'a> {
    stack: Vec<&'a Tree>,
}

impl<'a> Iter<'a> {
    pub fn new(root: &'a Node) -> Iter<'a> {
        Iter {
            stack: root.children.iter().collect()
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Point2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let node = self.stack.pop().unwrap();

            match node {
                Tree::Empty => (),
                Tree::Leaf(pt) => return Some(pt),
                Tree::Node(child) => self.stack.extend(&child.children[..]),
            }
        }

        None
    }
}

impl<'a> FusedIterator for Iter<'a> {}
