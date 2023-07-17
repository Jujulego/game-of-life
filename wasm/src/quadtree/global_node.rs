use std::slice::Iter;
use na::Point2;
use crate::quadtree::node::Node;
use crate::quadtree::quarter::{global_quarter, Quarter};
use crate::quadtree::tree::Tree;

/// Quadtree global node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalNode {
    pub children: [Tree; 4],
}

impl GlobalNode {
    /// Create a new empty node
    pub fn new() -> GlobalNode {
        GlobalNode {
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }
}

impl Node for GlobalNode {
    #[inline]
    fn quarter(&self, point: &Point2<i32>) -> Quarter {
        global_quarter(point)
    }

    #[inline]
    fn children(&self) -> Iter<'_, Tree> {
        self.children.iter()
    }

    #[inline]
    fn get_child(&self, quarter: Quarter) -> &Tree {
        unsafe { self.children.get_unchecked(quarter as usize) }
    }

    #[inline]
    fn get_child_mut(&mut self, quarter: Quarter) -> &mut Tree {
        unsafe { self.children.get_unchecked_mut(quarter as usize) }
    }
}
