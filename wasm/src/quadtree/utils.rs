use std::cmp::{max, min};
use na::{point, Point2, vector};
use py::BBox;
use py::traits::BBoxBounded;

/// Build bbox around a single point
pub fn bbox_around(pt: &Point2<i32>) -> BBox<i32, 2> {
    BBox::from_anchor_size(pt, &vector![1, 1])
}

/// Computes bbox of parent node
pub fn parent_bbox(child: &BBox<i32, 2>) -> BBox<i32, 2> {
    let start = child.start_point();
    let end = child.end_point();

    let size = (end - start) * 2;
    let start = point![start.x & !(size.x - 1), start.y & !(size.y - 1)];

    BBox::from_anchor_size(&start, &size)
}

/// Compute smallest common bbox
pub fn common_bbox(a: &Point2<i32>, b: &Point2<i32>) -> BBox<i32, 2> {
    if a.x.signum() != b.x.signum() || a.y.signum() != b.y.signum() {
        return (..).bbox();
    }

    let mut a = bbox_around(a);
    let mut b = bbox_around(b);

    while a != b {
        a = parent_bbox(&a);
        b = parent_bbox(&b);
    }

    a
}

fn next_power_of_2(mut v: i32) -> i32 {
    v = v.abs() - 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

#[cfg(test)]
mod tests {
    use py::traits::BBoxBounded;
    use super::*;

    #[test]
    fn test_bbox_around() {
        assert_eq!(bbox_around(&point![5, 5]), (point![5, 5]..point![6, 6]).bbox());
    }

    mod parent_bbox {
        use super::*;

        #[test]
        fn test_on_unit_bbox() {
            assert_eq!(
                parent_bbox(&(point![5, 5]..point![6, 6]).bbox()),
                (point![4, 4]..point![6, 6]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![5, 7]..point![6, 8]).bbox()),
                (point![4, 6]..point![6, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![7, 5]..point![8, 6]).bbox()),
                (point![6, 4]..point![8, 6]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![7, 7]..point![8, 8]).bbox()),
                (point![6, 6]..point![8, 8]).bbox()
            );
        }

        #[test]
        fn test_on_negative_unit_bbox() {
            assert_eq!(
                parent_bbox(&(point![-5, -5]..point![-4, -4]).bbox()),
                (point![-6, -6]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-5, -7]..point![-4, -6]).bbox()),
                (point![-6, -8]..point![-4, -6]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-7, -5]..point![-6, -4]).bbox()),
                (point![-8, -6]..point![-6, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-7, -7]..point![-6, -6]).bbox()),
                (point![-8, -8]..point![-6, -6]).bbox()
            );
        }

        #[test]
        fn test_on_bbox() {
            assert_eq!(
                parent_bbox(&(point![4, 4]..point![6, 6]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![4, 6]..point![6, 8]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![6, 4]..point![8, 6]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![6, 6]..point![8, 8]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
        }

        #[test]
        fn test_on_negative_bbox() {
            assert_eq!(
                parent_bbox(&(point![-6, -6]..point![-4, -4]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-6, -8]..point![-4, -6]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-8, -6]..point![-6, -4]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-8, -8]..point![-6, -6]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
        }
    }

    mod common_bbox {
        use super::*;

        #[test]
        fn test_on_positive() {
            assert_eq!(
                common_bbox(&point![3, 3], &point![5, 5]),
                (point![0, 0]..point![8, 8]).bbox()
            );
            assert_eq!(
                common_bbox(&point![3, 5], &point![5, 3]),
                (point![0, 0]..point![8, 8]).bbox()
            );
        }

        #[test]
        fn test_on_negative() {
            assert_eq!(
                common_bbox(&point![-3, -3], &point![-5, -5]),
                (point![-8, -8]..point![0, 0]).bbox()
            );
            assert_eq!(
                common_bbox(&point![-3, -5], &point![-5, -3]),
                (point![-8, -8]..point![0, 0]).bbox()
            );
        }

        #[test]
        fn test_different_signs() {
            assert_eq!(
                common_bbox(&point![-3, 3], &point![-5, -5]),
                (..).bbox()
            );
            assert_eq!(
                common_bbox(&point![3, -3], &point![-5, -5]),
                (..).bbox()
            );
        }
    }

    mod next_power_of_2 {
        use super::*;

        #[test]
        fn test_on_positive() {
            assert_eq!(next_power_of_2(3), 4);
            assert_eq!(next_power_of_2(5), 8);
            assert_eq!(next_power_of_2(12), 16);
        }

        #[test]
        fn test_on_negative() {
            assert_eq!(next_power_of_2(-3), 4);
            assert_eq!(next_power_of_2(-5), 8);
            assert_eq!(next_power_of_2(-12), 16);
        }
    }
}
