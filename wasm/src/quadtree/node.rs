use std::mem;
use std::slice::Iter;
use na::Point2;
use py::Holds;
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::quarter::Quarter;
use crate::quadtree::square_node::SquareNode;
use crate::quadtree::tree::Tree;

/// Quadtree node
pub trait Node {
    fn quarter(&self, point: &Point2<i32>) -> Quarter;

    fn children(&self) -> Iter<'_, Tree>;

    fn get_child(&self, quarter: Quarter) -> &Tree;
    fn get_child_mut(&mut self, quarter: Quarter) -> &mut Tree;

    /// Test if node contains point
    fn has(&self, point: &Point2<i32>) -> bool {
        match self.get_child(self.quarter(point)) {
            Tree::Empty => false,
            Tree::Leaf(pt) => point == pt,
            Tree::Node(child) => child.area.holds(point) && child.has(point),
        }
    }

    /// Search greatest node matching area
    fn search(&self, area: &BinarySquare) -> Option<&Tree> {
        let tree = self.get_child(self.quarter(&area.anchor));

        match tree {
            Tree::Empty => None,
            Tree::Leaf(pt) => {
                if area.holds(pt) {
                    Some(tree)
                } else {
                    None
                }
            },
            Tree::Node(child) => {
                if area.holds(&child.area) {
                    Some(tree)
                } else {
                    child.search(area)
                }
            },
        }
    }

    /// Insert new element in node
    fn insert(&mut self, element: Tree, at: &BinarySquare) {
        let pos = self.get_child_mut(self.quarter(&at.anchor));

        if &element == pos {
            return;
        }

        match pos {
            Tree::Empty => *pos = element,
            &mut Tree::Leaf(pt) => {
                let area = BinarySquare::wrapping(pt);
                let mut upper = Box::new(SquareNode::new(BinarySquare::common(&area, at).unwrap()));

                upper.insert(mem::replace(pos, Tree::Empty), &area);
                upper.insert(element, at);

                *pos = Tree::Node(upper);
            },
            Tree::Node(node) => {
                if node.area.holds(at) {
                    node.insert(element, at);
                } else {
                    let area = node.area;
                    let mut upper = Box::new(SquareNode::new(BinarySquare::common(&area, at).unwrap()));

                    upper.insert(mem::replace(pos, Tree::Empty), &area);
                    upper.insert(element, at);

                    *pos = Tree::Node(upper);
                }
            }
        }
    }

    /// Removes point from node
    fn remove(&mut self, point: &Point2<i32>) {
        let pos = self.get_child_mut(self.quarter(point));

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
