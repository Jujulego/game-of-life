use std::slice::Iter;
use na::Point2;
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

/// Quadtree node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SquareNode {
    pub area: BinarySquare,
    pub children: [Tree; 4],
}

impl SquareNode {
    /// Create a new empty node
    #[inline]
    pub fn new(area: BinarySquare) -> SquareNode {
        SquareNode {
            area,
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }
}

impl Node for SquareNode {
    #[inline]
    fn children(&self) -> Iter<'_, Tree> {
        self.children.iter()
    }

    #[inline]
    fn child_holding(&self, point: &Point2<i32>) -> &Tree {
        let idx = self.area.quarter(point) as usize;
        unsafe { self.children.get_unchecked(idx) }
    }

    #[inline]
    fn child_holding_mut(&mut self, point: &Point2<i32>) -> &mut Tree {
        let idx = self.area.quarter(point) as usize;
        unsafe { self.children.get_unchecked_mut(idx) }
    }
}
