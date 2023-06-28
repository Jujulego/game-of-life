use std::ops::Bound::{self, *};
use std::slice::Iter;
use na::Point2;
use py::BBox;
use crate::utils::cmp_xy_order;
use crate::xy_generator::XYGenerator;

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
    pub fn search(&self, area: BBox<i32, 2>) -> Vec<Point2<i32>> {
        let generator = XYGenerator::within(area);

        let mut result = Vec::new();
        let mut point = generator.first();
        let mut slice = self.elements.as_slice();

        while !slice.is_empty() {
            if slice[0] == point {
                result.push(point);
                slice = &slice[1..];
            } else {
                // Search point
                let res = slice.binary_search_by(|pt| cmp_xy_order(pt, &point));

                // Handle result
                match res {
                    Ok(idx) => {
                        result.push(point);
                        slice = &slice[idx + 1..];
                    },
                    Err(idx) => {
                        if idx >= slice.len() {
                            break;
                        }

                        slice = &slice[idx..];
                    }
                }
            }

            // Compute next point
            if let Some(pt) = generator.next(&point) {
                point = pt;
            } else {
                break;
            }
        }

        result
    }

    /// Insert point inside tree (if missing)
    pub fn insert(&mut self, point: &Point2<i32>) {
        let res = self.elements
            .binary_search_by(|pt| cmp_xy_order(pt, point));

        if let Err(idx) = res {
            self.elements.insert(idx, *point);
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
}

#[cfg(test)]
mod tests {
    use na::point;
    use py::traits::BBoxBounded;
    use super::*;

    #[test]
    fn test_quadtree_search() {
        let mut quadtree = BinaryTree::new();
        let area = (point![5, 5]..=point![10, 10]).bbox();

        quadtree.insert(&point![0, 5]);
        quadtree.insert(&point![5, 5]);
        quadtree.insert(&point![5, 10]);
        quadtree.insert(&point![5, 15]);
        quadtree.insert(&point![10, 5]);
        quadtree.insert(&point![10, 10]);
        quadtree.insert(&point![15, 10]);

        assert_eq!(
            quadtree.search(area),
            vec![
                point![5, 5],
                point![5, 10],
                point![10, 5],
                point![10, 10],
            ]
        );
    }
}
