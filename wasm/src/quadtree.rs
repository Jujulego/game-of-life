use std::fmt::Debug;
use na::Point2;
use py::{Holds, Overlaps};
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::global_node::GlobalNode;
use crate::quadtree::iter::Iter;
use crate::quadtree::node::Node;
use crate::quadtree::query::Query;
use crate::quadtree::tree::Tree;

mod binary_square;
mod global_node;
mod iter;
mod node;
mod quarter;
mod query;
mod square_node;
mod tree;

/// Quadtree wrapper
#[derive(Clone, Debug)]
pub struct Quadtree<N: Node> {
    root: N,
}

impl<N: Node> Quadtree<N> {
    #[inline]
    pub fn has(&self, point: &Point2<i32>) -> bool {
        self.root.has(point)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.root)
    }
}

pub type GlobalQuadtree = Quadtree<GlobalNode>;

impl Quadtree<GlobalNode> {
    pub fn new() -> Quadtree<GlobalNode> {
        Quadtree {
            root: GlobalNode::new(),
        }
    }

    pub fn query<B: Clone + Holds<Point2<i32>> + Overlaps<BinarySquare>>(&self, bbox: &B) -> Query<B> {
        Query::new(bbox, &self.root)
    }

    #[inline]
    pub fn insert(&mut self, point: Point2<i32>) {
        self.root.insert(Tree::Leaf(point), &BinarySquare::wrapping(point));
    }

    #[inline]
    pub fn remove(&mut self, point: &Point2<i32>) {
        self.root.remove(point);
    }
}

// Utils
impl Default for Quadtree<GlobalNode> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, N: Node> IntoIterator for &'a Quadtree<N> {
    type Item = &'a Point2<i32>;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use crate::quadtree::square_node::SquareNode;
    use super::*;

    #[test]
    fn test_has_point() {
        // Initiate tree
        let mut tree = Quadtree::default();
        tree.insert(point![3, 1]);
        tree.insert(point![3, 3]);
        tree.insert(point![3, 5]);

        // Inserted points
        assert!(tree.has(&point![3, 1]));
        assert!(tree.has(&point![3, 3]));
        assert!(tree.has(&point![3, 5]));

        // Others
        assert!(!tree.has(&point![0, 0]));
        assert!(!tree.has(&point![12, 42]));
    }

    #[test]
    fn test_iterator() {
        // Initiate tree
        let mut tree = Quadtree::default();
        tree.insert(point![3, 1]);
        tree.insert(point![3, 3]);
        tree.insert(point![3, 5]);

        // Inserted points
        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some(&point![3, 5]));
        assert_eq!(iter.next(), Some(&point![3, 3]));
        assert_eq!(iter.next(), Some(&point![3, 1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_point() {
        // Initiate tree
        let mut tree = Quadtree::default();

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        tree.insert(point![3, 1]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1])
                ]
            }
        );

        // Create a middle node
        tree.insert(point![1, 3]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3]),
                            Tree::Leaf(point![3, 1]),
                            Tree::Empty
                        ]
                    }))
                ]
            }
        );

        // Insert in middle node
        tree.insert(point![3, 3]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3]),
                            Tree::Leaf(point![3, 1]),
                            Tree::Leaf(point![3, 3]),
                        ]
                    }))
                ]
            }
        );

        // Move the middle node deeper
        tree.insert(point![3, 5]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 8 },
                        children: [
                            Tree::Node(Box::new(SquareNode {
                                area: BinarySquare { anchor: point![0, 0], size: 4 },
                                children: [
                                    Tree::Empty,
                                    Tree::Leaf(point![1, 3]),
                                    Tree::Leaf(point![3, 1]),
                                    Tree::Leaf(point![3, 3]),
                                ]
                            })),
                            Tree::Leaf(point![3, 5]),
                            Tree::Empty,
                            Tree::Empty,
                        ],
                    })),
                ]
            }
        );
    }

    #[test]
    fn test_insert_twice() {
        // Initiate tree
        let mut tree = Quadtree::default();

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        tree.insert(point![3, 1]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1]),
                ]
            }
        );

        // Insert again point
        tree.insert(point![3, 1]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1]),
                ]
            }
        );
    }

    #[test]
    fn test_remove_point() {
        // Initiate tree
        let mut tree = Quadtree::default();
        tree.insert(point![3, 1]);
        tree.insert(point![3, 3]);
        tree.insert(point![1, 3]);
        tree.insert(point![3, 5]);

        // Remove point
        tree.remove(&point![3, 3]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 8 },
                        children: [
                            Tree::Node(Box::new(SquareNode {
                                area: BinarySquare { anchor: point![0, 0], size: 4 },
                                children: [
                                    Tree::Empty,
                                    Tree::Leaf(point![1, 3]),
                                    Tree::Leaf(point![3, 1]),
                                    Tree::Empty
                                ]
                            })),
                            Tree::Leaf(point![3, 5]),
                            Tree::Empty,
                            Tree::Empty,
                        ],
                    })),
                ]
            }
        );

        // Simplify by moving node up
        tree.remove(&point![3, 5]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3]),
                            Tree::Leaf(point![3, 1]),
                            Tree::Empty
                        ]
                    })),
                ]
            }
        );

        // Simplify by moving point up
        tree.remove(&point![1, 3]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1]),
                ]
            }
        );
    }
}
