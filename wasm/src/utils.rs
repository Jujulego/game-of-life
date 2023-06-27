use std::cmp;
use std::ops::Bound;
use na::{point, Point2, vector, Vector2};
use py::BBox;

pub const DIRECTIONS: [Vector2<i32>; 8] = [
    vector![-1, -1],
    vector![-1,  0],
    vector![-1,  1],
    vector![ 0,  1],
    vector![ 1,  1],
    vector![ 1,  0],
    vector![ 1, -1],
    vector![ 0, -1],
];

/// Compare point in Z-order
/// => https://en.wikipedia.org/wiki/Z-order_curve
pub fn cmp_zorder(lhs: &Point2<i32>, rhs: &Point2<i32>) -> cmp::Ordering {
    let mut msd = 0;

    for dim in 0..2 {
        let a = lhs[msd] ^ rhs[msd];
        let b = lhs[dim] ^ rhs[dim];

        if a < b && a < (a ^ b) {
            msd = dim
        }
    }

    lhs[msd].cmp(&rhs[msd])
}

pub fn cmp_xy_order(lhs: &Point2<i32>, rhs: &Point2<i32>) -> cmp::Ordering {
    lhs.iter().cmp(rhs.iter())
}
