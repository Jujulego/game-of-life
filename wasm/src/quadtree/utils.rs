use std::cmp::{max, min};
use na::{point, Point2, vector};
use num_traits::Bounded;
use py::{BBox, PointBounds};

/// Build bbox around a single point
pub fn bbox_around(pt: &Point2<i32>) -> BBox<i32, 2> {
    BBox::from_anchor_size(pt, &vector![1, 1])
}

/// Computes bbox of parent node
pub fn parent_bbox(child: &BBox<i32, 2>) -> BBox<i32, 2> {
    let start = child.start_point().unwrap_or(Point2::min_value());
    let end = child.end_point().unwrap_or(Point2::max_value());

    let size = (end - start) * 2;
    let start = point![start.x & !(size.x - 1), start.y & !(size.y - 1)];

    BBox::from_anchor_size(&start, &size)
}

/// Compute smallest common bbox
pub fn common_bbox(a: &Point2<i32>, b: &Point2<i32>) -> BBox<i32, 2> {
    if a.x.signum() != b.x.signum() || a.y.signum() != b.y.signum() {
        return BBox::from(..);
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
    use super::*;

    #[test]
    fn test_bbox_around() {
        assert_eq!(bbox_around(&point![5, 5]), BBox::from(point![5, 5]..point![6, 6]));
    }

    mod parent_bbox {
        use super::*;

        #[test]
        fn test_on_unit_bbox() {
            assert_eq!(
                parent_bbox(&BBox::from(point![5, 5]..point![6, 6])),
                BBox::from(point![4, 4]..point![6, 6])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![5, 7]..point![6, 8])),
                BBox::from(point![4, 6]..point![6, 8])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![7, 5]..point![8, 6])),
                BBox::from(point![6, 4]..point![8, 6])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![7, 7]..point![8, 8])),
                BBox::from(point![6, 6]..point![8, 8])
            );
        }

        #[test]
        fn test_on_negative_unit_bbox() {
            assert_eq!(
                parent_bbox(&BBox::from(point![-5, -5]..point![-4, -4])),
                BBox::from(point![-6, -6]..point![-4, -4])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![-5, -7]..point![-4, -6])),
                BBox::from(point![-6, -8]..point![-4, -6])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![-7, -5]..point![-6, -4])),
                BBox::from(point![-8, -6]..point![-6, -4])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![-7, -7]..point![-6, -6])),
                BBox::from(point![-8, -8]..point![-6, -6])
            );
        }

        #[test]
        fn test_on_bbox() {
            assert_eq!(
                parent_bbox(&BBox::from(point![4, 4]..point![6, 6])),
                BBox::from(point![4, 4]..point![8, 8])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![4, 6]..point![6, 8])),
                BBox::from(point![4, 4]..point![8, 8])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![6, 4]..point![8, 6])),
                BBox::from(point![4, 4]..point![8, 8])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![6, 6]..point![8, 8])),
                BBox::from(point![4, 4]..point![8, 8])
            );
        }

        #[test]
        fn test_on_negative_bbox() {
            assert_eq!(
                parent_bbox(&BBox::from(point![-6, -6]..point![-4, -4])),
                BBox::from(point![-8, -8]..point![-4, -4])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![-6, -8]..point![-4, -6])),
                BBox::from(point![-8, -8]..point![-4, -4])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![-8, -6]..point![-6, -4])),
                BBox::from(point![-8, -8]..point![-4, -4])
            );
            assert_eq!(
                parent_bbox(&BBox::from(point![-8, -8]..point![-6, -6])),
                BBox::from(point![-8, -8]..point![-4, -4])
            );
        }
    }

    mod common_bbox {
        use super::*;

        #[test]
        fn test_on_positive() {
            assert_eq!(
                common_bbox(&point![3, 3], &point![5, 5]),
                BBox::from(point![0, 0]..point![8, 8])
            );
            assert_eq!(
                common_bbox(&point![3, 5], &point![5, 3]),
                BBox::from(point![0, 0]..point![8, 8])
            );
        }

        #[test]
        fn test_on_negative() {
            assert_eq!(
                common_bbox(&point![-3, -3], &point![-5, -5]),
                BBox::from(point![-8, -8]..point![0, 0])
            );
            assert_eq!(
                common_bbox(&point![-3, -5], &point![-5, -3]),
                BBox::from(point![-8, -8]..point![0, 0])
            );
        }

        #[test]
        fn test_different_signs() {
            assert_eq!(
                common_bbox(&point![-3, 3], &point![-5, -5]),
                BBox::from(..)
            );
            assert_eq!(
                common_bbox(&point![3, -3], &point![-5, -5]),
                BBox::from(..)
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
