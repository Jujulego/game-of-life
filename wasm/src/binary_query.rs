use std::iter::FusedIterator;
use na::Point2;
use py::{BBoxWalker, Holds, Walkable};
use crate::utils::cmp_xy_order;

pub struct BinaryQuery<'a, B: Holds<Point2<i64>> + Walkable<i64, 2>> {
    area: B,
    slice: &'a [Point2<i64>],
    walker: BBoxWalker<i64, 2>,
    next: Option<Point2<i64>>,
}

impl<'a, B: Holds<Point2<i64>> + Walkable<i64, 2>> BinaryQuery<'a, B> {
    pub fn new(area: B, slice: &'a [Point2<i64>]) -> BinaryQuery<'a, B> {
        let generator = area.walk().unwrap();

        BinaryQuery {
            area,
            slice,
            next: Some(*generator.first()),
            walker: generator,
        }
    }

    fn update_next(&mut self, last: &Point2<i64>) {
        self.next = self.walker.next(last);
    }
}

impl<'a, B: Holds<Point2<i64>> + Walkable<i64, 2>> Iterator for BinaryQuery<'a, B> {
    type Item = &'a Point2<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.slice.is_empty() {
                return None;
            }

            if let Some(next) = self.next {
                let first = unsafe { self.slice.get_unchecked(0) };

                if self.area.holds(first) {
                    self.slice = unsafe { self.slice.get_unchecked(1..) };
                    self.update_next(first);

                    return Some(first);
                } else {
                    // Search point
                    let res = self.slice.binary_search_by(|pt| cmp_xy_order(pt, &next));

                    if let Ok(idx) = res {
                        let point = &self.slice[idx];

                        self.slice = unsafe { self.slice.get_unchecked(idx + 1..) };
                        self.update_next(point);

                        return Some(point);
                    } else {
                        self.slice = unsafe {
                            let idx = res.unwrap_err_unchecked();
                            self.slice.get_unchecked(idx..)
                        };

                        self.update_next(&next);
                    }
                }
            } else {
                return None;
            }
        }
    }
}

impl<'a, B: Holds<Point2<i64>> + Walkable<i64, 2>> FusedIterator for BinaryQuery<'a, B> {}
