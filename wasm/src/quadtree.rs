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

    /// Returns all elements inside the given area
    pub fn search(&self, area: BBox<i32, 2>) -> Vec<Point2<i32>> {
        let generator = XYGenerator::within(area);

        let mut result = Vec::new();
        let mut point = generator.first();
        let mut slice = &self.elements[..];

        while !slice.is_empty() {
            // Search point
            #[cfg(any(feature = "binary-search", feature = "binary-search-quick-pick"))]
            let res = slice.binary_search_by(|pt| cmp_xy_order(pt, &point));

            #[cfg(feature = "binary-search-first-is-next")]
            let res = {
                if slice[0] == point {
                    Ok(0)
                } else {
                    slice.binary_search_by(|pt| cmp_xy_order(pt, &point))
                }
            };

            #[cfg(feature = "binary-search-first-in-bbox")]
            let res = {
                if area.contains(&slice[0]) {
                    point = slice[0];
                    Ok(0)
                } else {
                    slice.binary_search_by(|pt| cmp_xy_order(pt, &point))
                }
            };

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

            // Quick pick firsts
            #[cfg(feature = "binary-search-quick-pick")]
            while !slice.is_empty() && area.contains(&slice[0]) {
                point = slice[0];
                slice = &slice[1..];

                result.push(point);
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
