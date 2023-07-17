use std::cmp::{max, min};
use std::mem;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use na::{Point, point, Point2};
use py::{Holds, Overlaps, PointBounds};
use py::traits::DimensionBounds;
use crate::quadtree::quarter::{global_quarter, Quarter};

/// Square with a power of 2 as size
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinarySquare {
    pub anchor: Point2<i32>,
    pub size: u32,
}

// Methods
impl BinarySquare {
    fn search_area(start: Point2<i32>, end: Point2<i32>, mut bits: u32) -> BinarySquare {
        while bits < u32::BITS {
            let mask = (u32::MAX << bits) as i32;

            let square = BinarySquare {
                anchor: point![start.x & mask, start.y & mask],
                size: 1 << bits
            };

            // Correct exact bound case
            if square.holds(&end) {
                return square;
            } else {
                bits += 1;
            }
        }

        panic!("unable to find matching binary square");
    }

    /// Returns smallest square containing given squares
    pub fn common(rhs: &BinarySquare, lhs: &BinarySquare) -> Result<BinarySquare, &'static str> {
        if global_quarter(&rhs.anchor) != global_quarter(&lhs.anchor) {
            Err("given squares does not belong to the same global quarter")
        } else {
            // Compute extreme points
            let start = point![min(rhs.anchor.x, lhs.anchor.x), min(rhs.anchor.y, lhs.anchor.y)];
            let end = point![max(rhs.anchor.x, lhs.anchor.x), max(rhs.anchor.y, lhs.anchor.y)];

            // Search common square
            let bits = max(rhs.size, lhs.size).trailing_zeros() + 1;

            Ok(Self::search_area(start, end, bits))
        }
    }

    #[inline]
    pub fn wrapping(point: Point2<i32>) -> BinarySquare {
        BinarySquare {
            anchor: point,
            size: 1,
        }
    }

    /// Returns quarter containing point
    pub fn quarter(&self, point: &Point2<i32>) -> Quarter {
        let mask = (self.size >> 1) as i32;
        let mut quarter = 0u8;

        if (point.x & mask) == mask {
            quarter |= 0b10;
        }

        if (point.y & mask) == mask {
            quarter |= 0b01;
        }

        unsafe { mem::transmute(quarter) }
    }
}

// Utils
impl DimensionBounds<i32, 2> for BinarySquare {
    type Output = Range<i32>;

    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        let start = *self.anchor.get_unchecked(idx);

        start..(start + self.size as i32)
    }
}

impl PointBounds<i32, 2> for BinarySquare {
    #[inline]
    fn start_point(&self) -> Option<Point<i32, 2>> {
        Some(self.anchor)
    }

    #[inline]
    fn end_point(&self) -> Option<Point<i32, 2>> {
        let size = self.size as i32;

        Some(point![self.anchor.x + size, self.anchor.y + size])
    }
}

impl Holds<Point2<i32>> for BinarySquare {
    fn holds(&self, object: &Point2<i32>) -> bool {
        let left = self.size as i32;

        self.anchor.iter()
            .zip(object.iter())
            .all(|(a, o)| (a ^ o) < left)
    }
}

impl Holds<BinarySquare> for BinarySquare {
    #[inline]
    fn holds(&self, object: &BinarySquare) -> bool {
        object.size <= self.size && self.holds(&object.anchor)
    }
}

impl Overlaps<BinarySquare> for Range<Point2<i32>> {
    #[inline]
    fn overlaps(&self, lhs: &BinarySquare) -> bool {
        self.overlaps(&Range::from(lhs))
    }
}

impl Overlaps<BinarySquare> for RangeFrom<Point2<i32>> {
    #[inline]
    fn overlaps(&self, lhs: &BinarySquare) -> bool {
        self.overlaps(&Range::from(lhs))
    }
}

impl Overlaps<BinarySquare> for RangeFull {
    #[inline]
    fn overlaps(&self, _: &BinarySquare) -> bool {
        true
    }
}

impl Overlaps<BinarySquare> for RangeInclusive<Point2<i32>> {
    #[inline]
    fn overlaps(&self, lhs: &BinarySquare) -> bool {
        self.overlaps(&Range::from(lhs))
    }
}

impl Overlaps<BinarySquare> for RangeTo<Point2<i32>> {
    #[inline]
    fn overlaps(&self, lhs: &BinarySquare) -> bool {
        self.overlaps(&Range::from(lhs))
    }
}

impl Overlaps<BinarySquare> for RangeToInclusive<Point2<i32>> {
    #[inline]
    fn overlaps(&self, lhs: &BinarySquare) -> bool {
        self.overlaps(&Range::from(lhs))
    }
}

// Conversion
impl From<&BinarySquare> for Range<Point2<i32>> {
    fn from(value: &BinarySquare) -> Self {
        let size = value.size as i32;

        value.anchor..(point![value.anchor.x + size, value.anchor.y + size])
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    mod common {
        use super::*;

        #[test]
        fn test_area_area() {
            // Positive
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![6, 6], size: 2 }, &BinarySquare { anchor: point![4, 4], size: 1 }),
                Ok(BinarySquare { anchor: point![4, 4], size: 4 })
            );
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![2, 4], size: 2 }, &BinarySquare { anchor: point![6, 4], size: 1 }),
                Ok(BinarySquare { anchor: point![0, 0], size: 8 })
            );
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![4, 2], size: 2 }, &BinarySquare { anchor: point![4, 6], size: 1 }),
                Ok(BinarySquare { anchor: point![0, 0], size: 8 })
            );

            // Negative
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![-2, -4], size: 2 }, &BinarySquare { anchor: point![-6, -4], size: 1 }),
                Ok(BinarySquare { anchor: point![-8, -8], size: 8 })
            );
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![-4, -2], size: 2 }, &BinarySquare { anchor: point![-4, -6], size: 1 }),
                Ok(BinarySquare { anchor: point![-8, -8], size: 8 })
            );

            // Different global quarter
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![-6, -6], size: 2 }, &BinarySquare { anchor: point![8, 8], size: 1 }),
                Err("given squares does not belong to the same global quarter")
            );
        }

        #[test]
        fn test_strange() {
            assert_eq!(
                BinarySquare::common(&BinarySquare { anchor: point![130, 46], size: 2 }, &BinarySquare { anchor: point![133, 47], size: 1 }),
                Ok(BinarySquare { anchor: point![128, 40], size: 8 })
            );
        }
    }

    mod quarter {
        use super::*;

        #[test]
        fn test_positive_points() {
            let area = BinarySquare { anchor: point![0, 0], size: 4 };

            assert_eq!(area.quarter(&point![3, 3]), Quarter::NorthEast);
            assert_eq!(area.quarter(&point![3, 1]), Quarter::SouthEast);
            assert_eq!(area.quarter(&point![1, 3]), Quarter::NorthWest);
            assert_eq!(area.quarter(&point![1, 1]), Quarter::SouthWest);
        }

        #[test]
        fn test_negative_points() {
            let area = BinarySquare { anchor: point![-4, -4], size: 4 };

            assert_eq!(area.quarter(&point![-3, -3]), Quarter::SouthWest);
            assert_eq!(area.quarter(&point![-3, -1]), Quarter::NorthWest);
            assert_eq!(area.quarter(&point![-1, -3]), Quarter::SouthEast);
            assert_eq!(area.quarter(&point![-1, -1]), Quarter::NorthEast);
        }
    }

    mod holds {
        use na::point;
        use super::*;

        #[test]
        fn test_positive_point() {
            let square = BinarySquare {
                anchor: point![2, 2],
                size: 2
            };

            // Far outside
            assert!(!square.holds(&point![1, 1])); // below left
            assert!(!square.holds(&point![3, 1])); // left
            assert!(!square.holds(&point![5, 1])); // over left
            assert!(!square.holds(&point![1, 3])); // below
            assert!(!square.holds(&point![5, 3])); // over
            assert!(!square.holds(&point![1, 5])); // below right
            assert!(!square.holds(&point![3, 5])); // right
            assert!(!square.holds(&point![5, 5])); // over right

            // Border outside
            assert!(!square.holds(&point![2, 4]));
            assert!(!square.holds(&point![4, 4]));
            assert!(!square.holds(&point![4, 2]));

            // Inside
            assert!(square.holds(&point![2, 2]));
            assert!(square.holds(&point![3, 2]));
            assert!(square.holds(&point![2, 3]));
            assert!(square.holds(&point![3, 3]));
        }

        #[test]
        fn test_negative_point() {
            let square = BinarySquare {
                anchor: point![-4, -4],
                size: 2
            };

            // Far outside
            assert!(!square.holds(&point![-5, -5])); // below left
            assert!(!square.holds(&point![-3, -5])); // left
            assert!(!square.holds(&point![-1, -5])); // over left
            assert!(!square.holds(&point![-5, -3])); // below
            assert!(!square.holds(&point![-1, -3])); // over
            assert!(!square.holds(&point![-5, -1])); // below right
            assert!(!square.holds(&point![-3, -1])); // right
            assert!(!square.holds(&point![-1, -1])); // over right

            // Border outside
            assert!(!square.holds(&point![-4, -2]));
            assert!(!square.holds(&point![-2, -2]));
            assert!(!square.holds(&point![-2, -4]));

            // Inside
            assert!(square.holds(&point![-4, -4]));
            assert!(square.holds(&point![-3, -4]));
            assert!(square.holds(&point![-4, -3]));
            assert!(square.holds(&point![-3, -3]));
        }
    }
}
