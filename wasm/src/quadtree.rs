use std::mem;
use na::Point2;
use py::Holds;
use crate::quadtree::area::Area;
use crate::quadtree::division::Division;

mod area;
mod division;
mod quarter;
mod point;

/// Quadtree itself
#[derive(Clone, Debug, Eq, PartialEq)]
enum Tree {
    Leaf(Point2<i32>),
    Node(Box<Node>),
    Empty
}

/// Quadtree node
#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    area: Area,
    children: [Tree; 4],
}

impl Node {
    /// Create a new empty node
    fn new(area: Area) -> Node {
        Node {
            area,
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }

    fn has(&self, point: &Point2<i32>) -> bool {
        let idx = self.area.quarter(point) as usize;

        match &self.children[idx] {
            Tree::Empty => false,
            Tree::Leaf(pt) => {
                point == pt
            },
            Tree::Node(child) => {
                child.area.holds(point) && child.has(point)
            },
        }
    }

    fn insert<A: Division>(&mut self, element: Tree, at: &A) {
        let idx = self.area.quarter(at.anchor()) as usize;
        let current = mem::replace(&mut self.children[idx], Tree::Empty);

        match current {
            Tree::Empty => {
                self.children[idx] = element;
            },
            Tree::Leaf(pt) => {
                let mut node = Node::new(Area::common(&pt, at));

                node.insert(current, &pt);
                node.insert(element, at);

                self.children[idx] = Tree::Node(Box::new(node));
            },
            Tree::Node(mut child) => {
                if child.area.holds(at) {
                    child.insert(element, at);
                } else {
                    let area = child.area;
                    let mut node = Node::new(Area::common(&area, at));

                    node.insert(Tree::Node(child), &area);
                    node.insert(element, at);

                    self.children[idx] = Tree::Node(Box::new(node));
                }
            },
        }
    }
}

/// Quadtree wrapper
pub struct Quadtree {
    root: Node,
}

impl Quadtree {
    pub fn new() -> Quadtree {
        Quadtree {
            root: Node::new(Area::global())
        }
    }

    pub fn has(&self, point: &Point2<i32>) -> bool {
        self.root.has(point)
    }

    pub fn insert(&mut self, point: Point2<i32>) {
        self.root.insert(Tree::Leaf(point), &point);
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
        let mut tree = Quadtree::new();
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
    fn test_insert_point() {
        // Initiate tree
        let mut root = Node::new(Area::global());

        assert_eq!(
            root,
            Node {
                area: Area::global(),
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        let point = point![3, 1];
        root.insert(Tree::Leaf(point), &point);

        assert_eq!(
            root,
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
        let point = point![3, 3];
        root.insert(Tree::Leaf(point), &point);

        assert_eq!(
            root,
            Node {
                area: Area::global(),
                children: [
                    Tree::Node(Box::new(Node {
                        area: Area { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Leaf(point![3, 3]),
                            Tree::Empty,
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

        // Move the middle node
        let point = point![3, 5];
        root.insert(Tree::Leaf(point), &point);

        assert_eq!(
            root,
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
                                    Tree::Empty,
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
}
