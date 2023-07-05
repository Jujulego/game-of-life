use std::cmp::{max, min};
use na::{point, Point2};
use py::Holds;
use crate::quadtree::division::Division;
use crate::quadtree::quarter::{quarter, Quarter};

/// An area in the quadtree
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Area {
    pub anchor: Point2<i32>,
    pub size: u32,
}

impl Area {
    /// Area holding everything
    pub fn global() -> Area {
        Area {
            anchor: Point2::origin(),
            size: u32::MAX,
        }
    }

    /// Returns smallest area containing given division and point
    pub fn common<A: Division, B: Division>(a: &A, b: &B) -> Area {
        // Checks global quarters
        let global_quarter = quarter(&Point2::origin(), a.anchor());

        if global_quarter != quarter(&Point2::origin(), b.anchor()) {
            return Area::global();
        }

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

        let mut bits = u32::BITS - span.leading_zeros();

        while bits < u32::BITS {
            let mask = i32::MAX << bits;
            let size = 1 << bits;

            let anchor = point![start.x & mask, start.y & mask];

            // Correct exact bound case
            let overflow = max(
                end.x.abs_diff(anchor.x),
                end.y.abs_diff(anchor.y)
            );

            if overflow >= size {
                bits += 1;
            } else {
                return Area { anchor, size };
            }
        }

        Area::global()
    }

    fn center(&self) -> Point2<i32> {
        if self.size == u32::MAX {
            Point2::origin()
        } else {
            let delta = self.size as i32 / 2;

            point![
                self.anchor.x + delta,
                self.anchor.y + delta
            ]
        }
    }

    pub fn quarter(&self, point: &Point2<i32>) -> Quarter {
        let center = self.center();

        if point.x < center.x && point.y < center.y {
            Quarter::SouthWest
        } else if point.x < center.x {
            Quarter::NorthWest
        } else if point.y < center.y {
            Quarter::SouthEast
        } else {
            Quarter::NorthEast
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

    #[test]
    fn test_global_quarter() {
        assert_eq!(Area::global().quarter(&point![1, 1]), Quarter::NorthEast);
        assert_eq!(Area::global().quarter(&point![1, -1]), Quarter::SouthEast);
        assert_eq!(Area::global().quarter(&point![-1, 1]), Quarter::NorthWest);
        assert_eq!(Area::global().quarter(&point![-1, -1]), Quarter::SouthWest);
    }

    #[test]
    fn test_local_quarter() {
        let area = Area { anchor: point![0, 0], size: 4 };

        assert_eq!(area.quarter(&point![3, 3]), Quarter::NorthEast);
        assert_eq!(area.quarter(&point![3, 1]), Quarter::SouthEast);
        assert_eq!(area.quarter(&point![1, 3]), Quarter::NorthWest);
        assert_eq!(area.quarter(&point![1, 1]), Quarter::SouthWest);
    }

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
                Area { anchor: point![0, 0], size: 16 }
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

        #[test]
        fn test_area_point() {
            // Positive
            assert_eq!(
                Area::common(&Area { anchor: point![6, 6], size: 2 }, &point![4, 4]),
                Area { anchor: point![4, 4], size: 4 }
            );
            assert_eq!(
                Area::common(&Area { anchor: point![2, 4], size: 2 }, &point![6, 4]),
                Area { anchor: point![0, 0], size: 8 }
            );
            assert_eq!(
                Area::common(&Area { anchor: point![4, 2], size: 2 }, &point![4, 6]),
                Area { anchor: point![0, 0], size: 8 }
            );

            // Negative
            assert_eq!(
                Area::common(&Area { anchor: point![-2, -4], size: 2 }, &point![-6, -4]),
                Area { anchor: point![-8, -8], size: 8 }
            );
            assert_eq!(
                Area::common(&Area { anchor: point![-4, -2], size: 2 }, &point![-4, -6]),
                Area { anchor: point![-8, -8], size: 8 }
            );

            // Different quarter
            assert_eq!(
                Area::common(&Area { anchor: point![-6, -6], size: 2 }, &point![8, 8]),
                Area::global()
            );
        }

        #[test]
        fn test_strange() {
            assert_eq!(
                Area::common(&Area { anchor: point![130, 46], size: 2 }, &point![133, 47]),
                Area { anchor: point![128, 40], size: 8 }
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
