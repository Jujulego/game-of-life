use std::slice::Iter;
use na::Point2;
use crate::utils::cmp_xy_order;

/// Quadtree
#[derive(Clone)]
pub struct BinaryTree {
    elements: Vec<Point2<i32>>,
}

// Methods
impl BinaryTree {
    /// Creates an empty quadtree
    pub fn new() -> BinaryTree {
        BinaryTree {
            elements: Vec::new(),
        }
    }

    /// Returns iterator on elements
    pub fn iter(&self) -> Iter<'_, Point2<i32>> {
        self.elements.iter()
    }

    /// Returns true if quadtree contains point
    pub fn has(&self, point: &Point2<i32>) -> bool {
        self.elements
            .binary_search_by(|pt| cmp_xy_order(pt, point))
            .is_ok()
    }

    /// Insert point inside tree (if missing)
    pub fn insert(&mut self, point: Point2<i32>) {
        let res = self.elements
            .binary_search_by(|pt| cmp_xy_order(pt, &point));

        if let Err(idx) = res {
            self.elements.insert(idx, point);
        }
    }

    /// Removes point from tree (if present)
    pub fn remove(&mut self, point: &Point2<i32>) {
        let res = self.elements
            .binary_search_by(|pt| cmp_xy_order(pt, point));

        if let Ok(idx) = res {
            self.elements.remove(idx);
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }
}
