use std::ops::Bound::{self, *};
use na::{point, Point2};
use py::BBox;

/// Generates all points within a bbox
pub struct XYGenerator {
    area: BBox<i32, 2>
}

// Methods
impl XYGenerator {
    pub fn within(area: BBox<i32, 2>) -> XYGenerator {
        XYGenerator {
            area
        }
    }

    pub fn first(&self) -> Point2<i32> {
        point![first_in(&self.area[0]), first_in(&self.area[1])]
    }

    pub fn last(&self) -> Point2<i32> {
        point![last_in(&self.area[0]), last_in(&self.area[1])]
    }

    pub fn next(&self, pt: &Point2<i32>) -> Option<Point2<i32>> {
        let first = self.first();
        let last = self.last();

        if pt == &last || pt.x > last.x {
            None
        } else if pt.x < first.x {
            Some(first)
        } else if pt.y < first.y {
            Some(point![pt.x, first.y])
        } else if pt.y >= last.y {
            Some(point![pt.x + 1, first.y])
        } else {
            Some(point![pt.x, pt.y + 1])
        }
    }
}

// Utils
fn first_in(bounds: &(Bound<i32>, Bound<i32>)) -> i32 {
    if let Included(l) = bounds.0 {
        l
    } else if let Excluded(l) = bounds.0 {
        l + 1
    } else {
        i32::MIN
    }
}

fn last_in(bounds: &(Bound<i32>, Bound<i32>)) -> i32 {
    if let Included(l) = bounds.1 {
        l
    } else if let Excluded(l) = bounds.1 {
        l - 1
    } else {
        i32::MIN
    }
}

#[cfg(test)]
mod tests {
    use py::traits::BBoxBounded;
    use super::*;

    #[test]
    fn test_full_generation() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![0, 0]), Some(point![0, 1]));
        assert_eq!(generator.next(&point![0, 1]), Some(point![0, 2]));
        assert_eq!(generator.next(&point![0, 2]), Some(point![0, 3]));
        assert_eq!(generator.next(&point![0, 3]), Some(point![0, 4]));
        assert_eq!(generator.next(&point![0, 4]), Some(point![0, 5]));
        assert_eq!(generator.next(&point![0, 5]), Some(point![1, 0]));
        assert_eq!(generator.next(&point![1, 0]), Some(point![1, 1]));
        assert_eq!(generator.next(&point![1, 1]), Some(point![1, 2]));
        assert_eq!(generator.next(&point![1, 2]), Some(point![1, 3]));
        assert_eq!(generator.next(&point![1, 3]), Some(point![1, 4]));
        assert_eq!(generator.next(&point![1, 4]), Some(point![1, 5]));
        assert_eq!(generator.next(&point![1, 5]), Some(point![2, 0]));
        assert_eq!(generator.next(&point![2, 0]), Some(point![2, 1]));
        assert_eq!(generator.next(&point![2, 1]), Some(point![2, 2]));
        assert_eq!(generator.next(&point![2, 2]), Some(point![2, 3]));
        assert_eq!(generator.next(&point![2, 3]), Some(point![2, 4]));
        assert_eq!(generator.next(&point![2, 4]), Some(point![2, 5]));
        assert_eq!(generator.next(&point![2, 5]), Some(point![3, 0]));
        assert_eq!(generator.next(&point![3, 0]), Some(point![3, 1]));
        assert_eq!(generator.next(&point![3, 1]), Some(point![3, 2]));
        assert_eq!(generator.next(&point![3, 2]), Some(point![3, 3]));
        assert_eq!(generator.next(&point![3, 3]), Some(point![3, 4]));
        assert_eq!(generator.next(&point![3, 4]), Some(point![3, 5]));
        assert_eq!(generator.next(&point![3, 5]), Some(point![4, 0]));
        assert_eq!(generator.next(&point![4, 0]), Some(point![4, 1]));
        assert_eq!(generator.next(&point![4, 1]), Some(point![4, 2]));
        assert_eq!(generator.next(&point![4, 2]), Some(point![4, 3]));
        assert_eq!(generator.next(&point![4, 3]), Some(point![4, 4]));
        assert_eq!(generator.next(&point![4, 4]), Some(point![4, 5]));
        assert_eq!(generator.next(&point![4, 5]), Some(point![5, 0]));
        assert_eq!(generator.next(&point![5, 1]), Some(point![5, 2]));
        assert_eq!(generator.next(&point![5, 2]), Some(point![5, 3]));
        assert_eq!(generator.next(&point![5, 3]), Some(point![5, 4]));
        assert_eq!(generator.next(&point![5, 4]), Some(point![5, 5]));
        assert_eq!(generator.next(&point![5, 5]), None);
    }

    #[test]
    fn test_below_left_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![-2, -2]), Some(point![0, 0]));
    }

    #[test]
    fn test_left_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![-2, 2]), Some(point![0, 0]));
    }

    #[test]
    fn test_over_left_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![-2, 7]), Some(point![0, 0]));
    }

    #[test]
    fn test_below_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![2, -2]), Some(point![2, 0]));
    }

    #[test]
    fn test_over_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![2, 7]), Some(point![3, 0]));
    }

    #[test]
    fn test_last_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![5, 5]), None);
    }

    #[test]
    fn test_below_right_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![7, -2]), None);
    }

    #[test]
    fn test_right_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![7, 2]), None);
    }

    #[test]
    fn test_over_right_point() {
        let area = (point![0, 0]..=point![5, 5]).bbox();
        let generator = XYGenerator::within(area);

        assert_eq!(generator.next(&point![7, 7]), None);
    }
}
