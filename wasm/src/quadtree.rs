mod utils;

use na::Point2;
use py::BBox;

pub struct QuadDivision {
    bbox: BBox<i32, 2>,
    children: [Option<QuadNode>; 4],
}

pub enum QuadNode {
    Node(Box<QuadDivision>),
    Leaf(Point2<i32>),
}

pub struct QuadTree {
    root: Option<QuadNode>,
}

impl QuadTree {
    pub fn new() -> QuadTree {
        QuadTree {
            root: None,
        }
    }

    pub fn insert(&mut self, point: Point2<i32>) -> Result<(), String> {
        if let Some(node) = &self.root {
            match node {
                QuadNode::Node(div) => {
                    todo!("insert in division")
                },
                QuadNode::Leaf(pt) => {
                    todo!("divide leaf")
                }
            }
        } else {
            self.root = Some(QuadNode::Leaf(point));
        }

        Ok(())
    }
}
