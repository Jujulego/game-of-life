use na::Point2;
use crate::quadtree::node::Node;

/// Quadtree itself
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tree {
    Leaf(Point2<i32>),
    Node(Box<Node>),
    Empty
}
