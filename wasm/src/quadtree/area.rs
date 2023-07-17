use std::cmp::{max, min};
use std::collections::Bound::Excluded;
use std::ops::{Bound, Range};
use std::ops::Bound::{Included, Unbounded};
use na::{point, Point, Point2};
use py::{Holds, Intersection, Overlaps, PointBounds, Walkable};
use py::traits::DimensionBounds;
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

    fn search_area(start: &Point2<i32>, end: &Point2<i32>, mut bits: u32) -> Area {
        while bits < u32::BITS {
            let mask = (u32::MAX << bits) as i32;

            let area = Area {
                anchor: point![start.x & mask, start.y & mask],
                size: 1 << bits
            };

            // Correct exact bound case
            if area.holds(end) {
                return area;
            } else {
                bits += 1;
            }
        }

        Area::global()
    }

    /// Returns area surrounding given bbox
    pub fn surrounding<B: Walkable<i32, 2>>(bbox: &B) -> Area {
        let start = bbox.first_point();
        let end = bbox.last_point();

        match (&start, &end) {
            (Some(start), Some(end)) => {
                if quarter(&Point2::origin(), start) == quarter(&Point2::origin(), end) {
                    let size = end - start;
                    let bits = u32::BITS - max(size.x.unsigned_abs(), size.y.unsigned_abs()).leading_zeros();

                    Area::search_area(start, end, bits)
                } else {
                    Area::global()
                }
            },
            _ => Area::global()
        }
    }

    /// Builds area wrapping point
    pub fn wrapping(point: Point2<i32>) -> Area {
        Area {
            anchor: point,
            size: 1,
        }
    }

    /// Returns smallest area containing given division and point
    pub fn common(a: &Area, b: &Area) -> Area {
        // Checks global quarters
        let global_quarter = quarter(&Point2::origin(), &a.anchor);

        if global_quarter != quarter(&Point2::origin(), &b.anchor) {
            return Area::global()
        }

        // Extreme points
        let start = point![
            min(a.anchor.x, b.anchor.x),
            min(a.anchor.y, b.anchor.y)
        ];
        let end = point![
            max(a.anchor.x, b.anchor.x),
            max(a.anchor.y, b.anchor.y)
        ];

        // Compute common area
        let bits = max(a.size, b.size).trailing_zeros() + 1;

        Area::search_area(&start, &end, bits)
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

    fn as_range(&self) -> (Bound<Point2<i32>>, Bound<Point2<i32>>) {
        if self.size == u32::MAX {
            (Unbounded, Unbounded)
        } else {
            let size = self.size as i32;
            (Included(self.anchor), Excluded(Point2::new(self.anchor.x + size, self.anchor.y + size)))
        }
    }

    #[inline]
    pub fn quarter(&self, point: &Point2<i32>) -> Quarter {
        quarter(&self.center(), point)
    }
}

// Utils
impl DimensionBounds<i32, 2> for Area {
    type Output = (Bound<i32>, Bound<i32>);

    unsafe fn get_bounds_unchecked(&self, idx: usize) -> Self::Output {
        if self.size == u32::MAX {
            (Unbounded, Unbounded)
        } else {
            let start = *self.anchor.get_unchecked(idx);
            (Included(start), Excluded(start + self.size as i32))
        }
    }
}

impl PointBounds<i32, 2> for Area {
    #[inline]
    fn start_point(&self) -> Option<Point<i32, 2>> {
        if self.size == u32::MAX {
            None
        } else {
            Some(self.anchor)
        }
    }

    #[inline]
    fn end_point(&self) -> Option<Point<i32, 2>> {
        if self.size == u32::MAX {
            None
        } else {
            let size = self.size as i32;
            Some(Point2::new(self.anchor.x + size, self.anchor.y + size))
        }
    }
}

impl Holds<Area> for Area {
    fn holds(&self, object: &Area) -> bool {
        if self.size == u32::MAX {
            true
        } else if object.size > self.size {
            false
        } else {
            let left = self.size - object.size;

            self.anchor.iter()
                .zip(object.anchor.iter())
                .all(|(a, o)| (a ^ o) as u32 <= left)
        }
    }
}

impl Holds<Point2<i32>> for Area {
    fn holds(&self, object: &Point2<i32>) -> bool {
        if self.size == u32::MAX {
            true
        } else {
            let left = self.size - 1;

            self.anchor.iter()
                .zip(object.iter())
                .all(|(a, o)| (a ^ o) as u32 <= left)
        }
    }
}

impl<T, R> Intersection<T> for Area
where
    T: Clone + Intersection<(Bound<Point2<i32>>, Bound<Point2<i32>>), Output=R>,
    R: From<T>
{
    type Output = R;

    fn intersection(&self, rhs: &T) -> Self::Output {
        if self.size == u32::MAX {
            R::from(rhs.clone())
        } else {
            rhs.intersection(&self.as_range())
        }
    }
}

impl Overlaps<Area> for Range<Point2<i32>> {
    fn overlaps(&self, lhs: &Area) -> bool {
        if lhs.size == u32::MAX {
            true
        } else {
            let size = lhs.size as i32;

            self.start.x < (lhs.anchor.x + size) && self.start.y < (lhs.anchor.y + size) && self.end.x >= lhs.anchor.x && self.end.y >= lhs.anchor.y
        }
    }
}

// Conversions
impl From<&Area> for (Bound<Point2<i32>>, Bound<Point2<i32>>) {
    #[inline]
    fn from(value: &Area) -> Self {
        value.as_range()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use py::BBox;
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

    #[test]
    fn test_surrounding() {
        assert_eq!(
            Area::surrounding(&(point![1, 1]..=point![3, 3])),
            Area { anchor: point![0, 0], size: 4 }
        );
        assert_eq!(
            Area::surrounding(&(point![5, 1]..=point![7, 3])),
            Area { anchor: point![4, 0], size: 4 }
        );
        assert_eq!(
            Area::surrounding(&(point![1, 5]..=point![3, 7])),
            Area { anchor: point![0, 4], size: 4 }
        );
        assert_eq!(
            Area::surrounding(&(point![-1, -1]..=point![1, 1])),
            Area::global()
        );
    }

    #[test]
    fn test_intersection() {
        let area = Area { anchor: point![2, 2], size: 2 };

        assert_eq!(
            area.intersection(&(point![0, 0]..point![3, 3])),
            BBox::from(point![2, 2]..point![3, 3])
        );
        assert_eq!(
            area.intersection(&(point![0, 3]..point![3, 5])),
            BBox::from(point![2, 3]..point![3, 4])
        );
        assert_eq!(
            area.intersection(&(point![3, 0]..point![5, 3])),
            BBox::from(point![3, 2]..point![4, 3])
        );
        assert_eq!(
            area.intersection(&(point![3, 3]..point![5, 5])),
            BBox::from(point![3, 3]..point![4, 4])
        );
    }

    mod common {
        use super::*;

        #[test]
        fn test_area_area() {
            // Positive
            assert_eq!(
                Area::common(&Area { anchor: point![6, 6], size: 2 }, &Area { anchor: point![4, 4], size: 1 }),
                Area { anchor: point![4, 4], size: 4 }
            );
            assert_eq!(
                Area::common(&Area { anchor: point![2, 4], size: 2 }, &Area { anchor: point![6, 4], size: 1 }),
                Area { anchor: point![0, 0], size: 8 }
            );
            assert_eq!(
                Area::common(&Area { anchor: point![4, 2], size: 2 }, &Area { anchor: point![4, 6], size: 1 }),
                Area { anchor: point![0, 0], size: 8 }
            );

            // Negative
            assert_eq!(
                Area::common(&Area { anchor: point![-2, -4], size: 2 }, &Area { anchor: point![-6, -4], size: 1 }),
                Area { anchor: point![-8, -8], size: 8 }
            );
            assert_eq!(
                Area::common(&Area { anchor: point![-4, -2], size: 2 }, &Area { anchor: point![-4, -6], size: 1 }),
                Area { anchor: point![-8, -8], size: 8 }
            );

            // Different quarter
            assert_eq!(
                Area::common(&Area { anchor: point![-6, -6], size: 2 }, &Area { anchor: point![8, 8], size: 1 }),
                Area::global()
            );
        }

        #[test]
        fn test_strange() {
            assert_eq!(
                Area::common(&Area { anchor: point![130, 46], size: 2 }, &Area { anchor: point![133, 47], size: 1 }),
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
        fn test_area_holds_point_negative() {
            let area = Area {
                anchor: point![-4, -4],
                size: 2
            };

            // Far outside
            assert!(!area.holds(&point![-5, -5])); // below left
            assert!(!area.holds(&point![-3, -5])); // left
            assert!(!area.holds(&point![-1, -5])); // over left
            assert!(!area.holds(&point![-5, -3])); // below
            assert!(!area.holds(&point![-1, -3])); // over
            assert!(!area.holds(&point![-5, -1])); // below right
            assert!(!area.holds(&point![-3, -1])); // right
            assert!(!area.holds(&point![-1, -1])); // over right

            // Border outside
            assert!(!area.holds(&point![-4, -2]));
            assert!(!area.holds(&point![-2, -2]));
            assert!(!area.holds(&point![-2, -4]));

            // Inside
            assert!(area.holds(&point![-4, -4]));
            assert!(area.holds(&point![-3, -4]));
            assert!(area.holds(&point![-4, -3]));
            assert!(area.holds(&point![-3, -3]));
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
