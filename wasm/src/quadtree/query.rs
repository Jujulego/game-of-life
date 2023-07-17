use na::Point2;
use py::{Holds, Overlaps};
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

pub struct Query<'a, B: Holds<Point2<i32>>> {
    bbox: B,
    stack: Vec<&'a Tree>,
}

impl<'a, B: Clone + Holds<Point2<i32>> + Overlaps<BinarySquare>> Query<'a, B> {
    #[inline]
    pub fn new<N: Node>(bbox: &B, root: &'a N) -> Query<'a, B> {
        Query {
            bbox: bbox.clone(),
            stack: root.children().collect(),
        }
    }
}

impl<'a, B: Holds<Point2<i32>> + Overlaps<BinarySquare>> Iterator for Query<'a, B> {
    type Item = &'a Point2<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(Tree::Empty) => (),
                Some(Tree::Leaf(pt)) => {
                    if self.bbox.holds(pt) {
                        return Some(pt);
                    }
                },
                Some(Tree::Node(child)) => {
                    if self.bbox.overlaps(&child.area) {
                        self.stack.extend(&child.children)
                    }
                },
            }
        }
    }
}
