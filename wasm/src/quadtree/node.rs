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
        let pos = unsafe { self.children.get_unchecked_mut(idx) };

        match pos {
            Tree::Empty => *pos = element,
            &mut Tree::Leaf(pt) => {
                let mut upper = Box::new(Node::new(Area::common(&pt, at)));

                upper.insert(mem::replace(pos, Tree::Empty), &pt);
                upper.insert(element, at);

                *pos = Tree::Node(upper);
            },
            Tree::Node(node) => {
                if node.area.holds(at) {
                    node.insert(element, at);
                } else {
                    let area = node.area;
                    let mut upper = Box::new(Node::new(Area::common(&area, at)));

                    upper.insert(mem::replace(pos, Tree::Empty), &area);
                    upper.insert(element, at);

                    *pos = Tree::Node(upper);
                }
            }
        }
    }

    pub fn remove(&mut self, point: &Point2<i32>) {
        let idx = self.area.quarter(point) as usize;
        let pos = unsafe { self.children.get_unchecked_mut(idx) };

        match pos {
            Tree::Empty => (),
            Tree::Leaf(ref pt) => {
                if pt == point {
                    *pos = Tree::Empty;
                }
            }
            Tree::Node(node) => {
                if node.area.holds(point) {
                    node.remove(point);

                    let mut last = None;

                    for child in &mut node.children {
                        if child != &Tree::Empty {
                            if last.is_none() {
                                last = Some(child);
                            } else {
                                return;
                            }
                        }
                    }

                    if let Some(last) = last {
                        *pos = mem::replace(last, Tree::Empty);
                    } else {
                        *pos = Tree::Empty;
                    }
                }
            }
        }
    }
}
