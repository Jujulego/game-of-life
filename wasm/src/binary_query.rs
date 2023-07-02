use std::iter::FusedIterator;
use na::Point2;
use py::{BBox, Holds};
use crate::utils::cmp_xy_order;
use crate::xy_generator::XYGenerator;

pub struct BinaryQuery<'a> {
    area: BBox<i32, 2>,
    slice: &'a [Point2<i32>],
    generator: XYGenerator,
    next: Option<Point2<i32>>,
}

impl<'a> BinaryQuery<'a> {
    pub fn new(area: BBox<i32, 2>, slice: &'a [Point2<i32>]) -> BinaryQuery<'a> {
        let generator = XYGenerator::within(area);

        BinaryQuery {
            area,
            slice,
            next: Some(generator.first()),
            generator,
        }
    }

    fn update_next(&mut self, last: &Point2<i32>) {
        self.next = self.generator.next(last);
    }
}

impl<'a> Iterator for BinaryQuery<'a> {
    type Item = &'a Point2<i32>;

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

impl<'a> FusedIterator for BinaryQuery<'a> {}
