use std::cmp::{max, min};
use na::{point, Point2};
use py::Holds;
use crate::quadtree::division::Division;
use crate::quadtree::quarter::quarter;
use crate::quadtree::quarter::Quarter::NorthEast;

/// An area in the quadtree
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Area {
    pub(crate) anchor: Point2<i32>,
    pub(crate) size: u32,
}

impl Area {
    pub fn global() -> Area {
        Area {
            anchor: Point2::origin(),
            size: u32::MAX,
        }
    }

    pub fn common<A: Division, B: Division>(a: &A, b: &B) -> Area {
        // Checks global quarters
        let global_quarter = quarter(&Point2::origin(), a.anchor());

        if global_quarter != quarter(&Point2::origin(), b.anchor()) {
            Area::global()
        } else {
            // Extreme points
            let start = point![
                min(a.anchor().x, b.anchor().x),
                min(a.anchor().y, b.anchor().y)
            ];

            let end = point![
                max(a.anchor().x + a.size() as i32 - 1, b.anchor().x + b.size() as i32 - 1),
                max(a.anchor().y + a.size() as i32 - 1, b.anchor().y + b.size() as i32 - 1)
            ];

            // Compute common area
            let span = max(
                (end.x - start.x) as u32,
                (end.y - start.y) as u32,
            );

            let bits = u32::BITS - span.leading_zeros();
            let mask = i32::MAX << bits;

            let anchor= point![
                start.x & mask,
                start.y & mask
            ];
            let mut size = 1 << bits;

            // Correct max negative cases
            if global_quarter != NorthEast {
                let overflow = max(
                    end.x.abs_diff(anchor.x),
                    end.y.abs_diff(anchor.y)
                );

                if overflow == size {
                    size *= 2;
                }
            }

            Area { anchor, size }
        }
    }
}

// Utils
impl Division for Area {
    #[inline]
    fn anchor(&self) -> &Point2<i32> {
        &self.anchor
    }

    #[inline]
    fn size(&self) -> u32 {
        self.size
    }
}

impl<D: Division> Holds<D> for Area {
    fn holds(&self, object: &D) -> bool {
        if object.size() > self.size {
            false
        } else {
            let left = self.size.abs_diff(object.size());

            self.anchor.iter()
                .zip(object.anchor().iter())
                .all(|(a, o)| a <= o && o.abs_diff(*a) <= left)
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    mod common {
        use super::*;

        #[test]
        fn test_point_point() {
            // Positive
            assert_eq!(
                Area::common(&point![4, 4], &point![6, 6]),
                Area { anchor: point![4, 4], size: 4 }
            );
            assert_eq!(
                Area::common(&point![1, 1], &point![3, 3]),
                Area { anchor: point![0, 0], size: 4 }
            );
            assert_eq!(
                Area::common(&point![1, 3], &point![3, 1]),
                Area { anchor: point![0, 0], size: 4 }
            );
            assert_eq!(
                Area::common(&point![4, 4], &point![8, 8]),
                Area { anchor: point![0, 0], size: 8 }
            );

            // Negative
            assert_eq!(
                Area::common(&point![-7, -7], &point![-8, -8]),
                Area { anchor: point![-8, -8], size: 2 }
            );
            assert_eq!(
                Area::common(&point![-7, -7], &point![-6, -6]),
                Area { anchor: point![-8, -8], size: 4 }
            );

            // Different quarter
            assert_eq!(
                Area::common(&point![-7, -7], &point![8, 8]),
                Area::global()
            );
        }
    }

    mod holds {
        use na::point;
        use super::*;

        #[test]
        fn test_area_holds_point() {
            let area = Area {
                anchor: point![2, 2],
                size: 2
            };

            // Far outside
            assert!(!area.holds(&point![1, 1])); // below left
            assert!(!area.holds(&point![3, 1])); // left
            assert!(!area.holds(&point![5, 1])); // over left
            assert!(!area.holds(&point![1, 3])); // below
            assert!(!area.holds(&point![5, 3])); // over
            assert!(!area.holds(&point![1, 5])); // below right
            assert!(!area.holds(&point![3, 5])); // right
            assert!(!area.holds(&point![5, 5])); // over right

            // Border outside
            assert!(!area.holds(&point![2, 4]));
            assert!(!area.holds(&point![4, 4]));
            assert!(!area.holds(&point![4, 2]));

            // Inside
            assert!(area.holds(&point![2, 2]));
            assert!(area.holds(&point![3, 2]));
            assert!(area.holds(&point![2, 3]));
            assert!(area.holds(&point![3, 3]));
        }

        #[test]
        fn test_area_holds_area() {
            let area = Area {
                anchor: point![2, 2],
                size: 2
            };

            // Inside
            assert!(area.holds(&Area { anchor: point![2, 2], size: 1 }));
            assert!(area.holds(&Area { anchor: point![2, 2], size: 2 }));
            assert!(area.holds(&Area { anchor: point![2, 3], size: 1 }));
            assert!(area.holds(&Area { anchor: point![3, 2], size: 1 }));
            assert!(area.holds(&Area { anchor: point![3, 3], size: 1 }));

            // Outside
            assert!(!area.holds(&Area { anchor: point![2, 2], size: 3 }));
            assert!(!area.holds(&Area { anchor: point![2, 3], size: 2 }));
            assert!(!area.holds(&Area { anchor: point![3, 2], size: 2 }));
            assert!(!area.holds(&Area { anchor: point![3, 3], size: 2 }));
        }
    }
}
