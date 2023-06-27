use std::slice::Iter;
use na::Point2;
use py::BBox;
use crate::utils::cmp_xy_order;
use crate::xy_generator::XYGenerator;

/// Quadtree
#[derive(Clone)]
pub struct Quadtree {
    elements: Vec<Point2<i32>>,
}

// Methods
impl Quadtree {
    /// Creates an empty quadtree
    pub fn new() -> Quadtree {
        Quadtree {
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

    pub fn search(&self, area: &BBox<i32, 2>) -> Vec<Point2<i32>> {
        let generator = XYGenerator::within(area);

        let mut result = Vec::new();
        let mut point = generator.first();
        let mut slice = &self.elements[..];

        loop {
            dbg!(point);
            // Search point
            let res = slice.binary_search_by(|pt| cmp_xy_order(pt, &point));

            // Handle result
            match res {
                Ok(idx) => {
                    result.push(point);
                    slice = &slice[idx + 1..];
                }
                Err(idx) => {
                    if idx >= slice.len() {
                        break;
                    }

                    slice = &slice[idx..];
                }
            }

            // Compute next point
            match generator.next(&point) {
                Some(pt) => point = pt,
                None => break
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
        let mut quadtree = Quadtree::new();
        let area = (point![1, 1]..=point![2, 2]).bbox();

        quadtree.insert(&point![0, 1]);
        quadtree.insert(&point![1, 1]);
        quadtree.insert(&point![1, 2]);
        quadtree.insert(&point![1, 3]);
        quadtree.insert(&point![2, 1]);
        quadtree.insert(&point![2, 2]);
        quadtree.insert(&point![3, 2]);

        assert_eq!(
            quadtree.search(&area),
            vec![
                point![1, 1],
                point![1, 2],
                point![2, 1],
                point![2, 2],
            ]
        );
    }
}
