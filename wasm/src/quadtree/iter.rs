use std::iter::FusedIterator;
use na::Point2;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

pub struct Iter<'a> {
    stack: Vec<&'a Tree>,
}

impl<'a> Iter<'a> {
    pub fn new<N: Node>(root: &'a N) -> Iter<'a> {
        Iter {
            stack: root.children().collect()
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Point2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(Tree::Empty) => (),
                Some(Tree::Leaf(pt)) => return Some(pt),
                Some(Tree::Node(child)) => self.stack.extend(&child.children),
            }
        }
    }
}

impl<'a> FusedIterator for Iter<'a> {}
