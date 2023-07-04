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
#[derive(Clone, Debug)]
enum Tree {
    Leaf(Point2<i32>),
    Node(Box<Node>),
    Empty
}

/// Quadtree node
#[derive(Clone, Debug)]
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

    pub fn insert(&mut self, point: Point2<i32>) {
        self.root.insert(Tree::Leaf(point), &point);
    }
}
