use std::slice::Iter;
use na::Point2;
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::node::Node;
use crate::quadtree::quarter::Quarter;
use crate::quadtree::tree::Tree;

/// Quadtree node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SquareNode {
    pub area: BinarySquare,
    pub children: [Tree; 4],
}

impl SquareNode {
    /// Create a new empty node
    pub fn new(area: BinarySquare) -> SquareNode {
        SquareNode {
            area,
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }
}

impl Node for SquareNode {
    #[inline]
    fn quarter(&self, point: &Point2<i32>) -> Quarter {
        self.area.quarter(point)
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
