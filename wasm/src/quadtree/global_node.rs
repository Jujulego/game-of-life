use std::slice::Iter;
use na::Point2;
use crate::quadtree::node::Node;
use crate::quadtree::quarter::global_quarter;
use crate::quadtree::tree::Tree;

/// Quadtree global node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalNode {
    pub children: [Tree; 4],
}

impl GlobalNode {
    /// Create a new empty node
    #[inline]
    pub fn new() -> GlobalNode {
        GlobalNode {
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }
}

// Utils
impl Default for GlobalNode {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Node for GlobalNode {
    #[inline]
    fn children(&self) -> Iter<'_, Tree> {
        self.children.iter()
    }

    #[inline]
    fn child_holding(&self, point: &Point2<i32>) -> &Tree {
        let idx = global_quarter(point) as usize;
        unsafe { self.children.get_unchecked(idx) }
    }

    #[inline]
    fn child_holding_mut(&mut self, point: &Point2<i32>) -> &mut Tree {
        let idx = global_quarter(point) as usize;
        unsafe { self.children.get_unchecked_mut(idx) }
    }
}
