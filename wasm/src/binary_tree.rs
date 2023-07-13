use std::slice::Iter;
use na::Point2;
use py::{Holds, Walkable};
use crate::utils::cmp_xy_order;

#[cfg(feature = "binary-tree")]
use crate::binary_query::BinaryQuery;

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

    /// Returns all elements inside the given area
    #[cfg(feature = "binary-tree")]
    pub fn query<B: Holds<Point2<i32>> + Walkable<i32, 2>>(&self, area: B) -> BinaryQuery<'_, B> {
        BinaryQuery::new(area, self.elements.as_slice())
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

#[cfg(test)]
#[cfg(feature = "binary-tree")]
mod tests {
    use na::point;
    use super::*;

    #[test]
    fn test_tree_query() {
        let mut tree = BinaryTree::new();
        let area = point![5, 5]..=point![10, 10];

        tree.insert(point![0, 5]);
        tree.insert(point![5, 5]);
        tree.insert(point![5, 10]);
        tree.insert(point![5, 15]);
        tree.insert(point![10, 5]);
        tree.insert(point![10, 10]);
        tree.insert(point![15, 10]);

        assert_eq!(
            tree.query(area).copied().collect::<Vec<Point2<i32>>>(),
            vec![
                point![5, 5],
                point![5, 10],
                point![10, 5],
                point![10, 10],
            ]
        );
    }
}
