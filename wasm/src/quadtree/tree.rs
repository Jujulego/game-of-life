use na::Point2;
use crate::quadtree::square_node::SquareNode;

/// Quadtree itself
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tree {
    Leaf(Point2<i32>),
    Node(Box<SquareNode>),
    Empty
}
