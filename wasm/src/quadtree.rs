use std::fmt::Debug;
use na::Point2;
use py::{Holds, Intersection, Walkable};
use crate::quadtree::area::Area;
use crate::quadtree::iter::Iter;
use crate::quadtree::node::Node;
use crate::quadtree::query::Query;
use crate::quadtree::tree::Tree;
use crate::traits::overlap::Overlaps;

mod area;
mod division;
mod iter;
mod node;
mod point;
mod quarter;
mod query;
mod tree;

/// Quadtree wrapper
#[derive(Clone, Debug)]
pub struct Quadtree {
    root: Node,
}

impl Quadtree {
    pub fn new() -> Quadtree {
        Quadtree {
            root: Node::new(Area::global())
        }
    }

    pub fn inside<B: Walkable<i32, 2>>(bbox: &B) -> Quadtree {
        Quadtree {
            root: Node::new(Area::surrounding(bbox))
        }
    }

    #[inline]
    pub fn has(&self, point: &Point2<i32>) -> bool {
        self.root.area.holds(point) && self.root.has(point)
    }

    pub fn query<R: Holds<Point2<i32>> + Overlaps<Area>, B: Intersection<Area, Output=R>>(&self, bbox_a: B) -> Query<R> {
        let bbox = bbox_a.intersection(&self.root.area);
        Query::new(bbox, &self.root)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self.root)
    }

    #[inline]
    pub fn insert(&mut self, point: Point2<i32>) {
        if self.root.area.holds(&point) {
            self.root.insert(Tree::Leaf(point), &point);
        } else {
            panic!("Cannot insert point {point}, it is outside of root node");
        }
    }

    #[inline]
    pub fn remove(&mut self, point: &Point2<i32>) {
        if self.root.area.holds(point) {
            self.root.remove(point);
        }
    }
}

// Utils
impl Default for Quadtree {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Quadtree {
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

        assert_eq!(iter.next(), Some(&point![3, 1]));
        assert_eq!(iter.next(), Some(&point![3, 3]));
        assert_eq!(iter.next(), Some(&point![3, 5]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_point() {
        // Initiate tree
        let mut tree = Quadtree::default();

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        tree.insert(point![3, 1]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Leaf(point![3, 1]),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );

        // Create a middle node
        tree.insert(point![1, 3]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Node(Box::new(Node {
                        area: Area { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3]),
                            Tree::Leaf(point![3, 1]),
                            Tree::Empty
                        ]
                    })),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );

        // Insert in middle node
        tree.insert(point![3, 3]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Node(Box::new(Node {
                        area: Area { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Leaf(point![3, 3]),
                            Tree::Leaf(point![1, 3]),
                            Tree::Leaf(point![3, 1]),
                            Tree::Empty
                        ]
                    })),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );

        // Move the middle node deeper
        tree.insert(point![3, 5]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Node(Box::new(Node {
                        area: Area { anchor: point![0, 0], size: 8 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![3, 5]),
                            Tree::Empty,
                            Tree::Node(Box::new(Node {
                                area: Area { anchor: point![0, 0], size: 4 },
                                children: [
                                    Tree::Leaf(point![3, 3]),
                                    Tree::Leaf(point![1, 3]),
                                    Tree::Leaf(point![3, 1]),
                                    Tree::Empty
                                ]
                            })),
                        ],
                    })),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
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
            Node {
                area: Area::global(),
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        tree.insert(point![3, 1]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Leaf(point![3, 1]),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );

        // Insert again point
        tree.insert(point![3, 1]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Leaf(point![3, 1]),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
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
            Node {
                area: Area::global(),
                children: [
                    Tree::Node(Box::new(Node {
                        area: Area { anchor: point![0, 0], size: 8 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![3, 5]),
                            Tree::Empty,
                            Tree::Node(Box::new(Node {
                                area: Area { anchor: point![0, 0], size: 4 },
                                children: [
                                    Tree::Empty,
                                    Tree::Leaf(point![1, 3]),
                                    Tree::Leaf(point![3, 1]),
                                    Tree::Empty
                                ]
                            })),
                        ],
                    })),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );

        // Simplify by moving node up
        tree.remove(&point![3, 5]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Node(Box::new(Node {
                        area: Area { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3]),
                            Tree::Leaf(point![3, 1]),
                            Tree::Empty
                        ]
                    })),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );

        // Simplify by moving point up
        tree.remove(&point![1, 3]);

        assert_eq!(
            tree.root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Leaf(point![3, 1]),
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty
                ]
            }
        );
    }
}
