use na::Point2;
use py::Holds;
use crate::quadtree::area::Area;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;
use crate::traits::overlap::Overlaps;

pub struct Query<'a, B: Holds<Point2<i64>>> {
    bbox: B,
    stack: Vec<&'a Tree>,
}

impl<'a, B: Holds<Point2<i64>> + Overlaps<Area>> Query<'a, B> {
    pub fn new(bbox: B, root: &'a Node) -> Query<'a, B> {
        let mut stack = Vec::new();
        stack.extend(&root.children);

        Query { bbox, stack }
    }
}

impl<'a, B: Holds<Point2<i64>> + Overlaps<Area>> Iterator for Query<'a, B> {
    type Item = &'a Point2<i64>;

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
                    if self.bbox.overlap(&child.area) {
                        self.stack.extend(&child.children)
                    }
                },
            }
        }
    }
}
