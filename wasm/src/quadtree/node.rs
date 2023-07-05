use std::mem;
use na::Point2;
use py::Holds;
use crate::quadtree::area::Area;
use crate::quadtree::division::Division;
use crate::quadtree::tree::Tree;

/// Quadtree node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    pub area: Area,
    pub children: [Tree; 4],
}

impl Node {
    /// Create a new empty node
    pub fn new(area: Area) -> Node {
        Node {
            area,
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }

    pub fn has(&self, point: &Point2<i32>) -> bool {
        let idx = self.area.quarter(point) as usize;

        match &self.children[idx] {
            Tree::Empty => false,
            Tree::Leaf(pt) => point == pt,
            Tree::Node(child) => child.area.holds(point) && child.has(point),
        }
    }

    pub fn insert<A: Division>(&mut self, element: Tree, at: &A) {
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
                    self.children[idx] = Tree::Node(child);
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

    fn children_count(&self) -> usize {
        self.children.iter()
            .filter(|&t| t != &Tree::Empty)
            .count()
    }

    fn extract_child(&mut self) -> Tree {
        for child in &mut self.children[..] {
            if child != &Tree::Empty {
                return mem::replace(child, Tree::Empty);
            }
        }

        Tree::Empty
    }

    pub fn remove(&mut self, point: &Point2<i32>) {
        let idx = self.area.quarter(point) as usize;

        match &mut self.children[idx] {
            Tree::Empty => (),
            Tree::Leaf(pt) => {
                if pt == point {
                    self.children[idx] = Tree::Empty;
                }
            }
            Tree::Node(child) => {
                if child.area.holds(point) {
                    child.remove(point);

                    match child.children_count() {
                        0 => self.children[idx] = Tree::Empty,
                        1 => self.children[idx] = child.extract_child(),
                        _ => ()
                    }
                }
            }
        }
    }
}
