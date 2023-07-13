use std::cmp;
use na::Point2;
use wasm_bindgen::prelude::wasm_bindgen;

/// Compare point in Z-order
/// => https://en.wikipedia.org/wiki/Z-order_curve
pub fn cmp_zorder(lhs: &Point2<i64>, rhs: &Point2<i64>) -> cmp::Ordering {
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

pub fn cmp_xy_order(lhs: &Point2<i64>, rhs: &Point2<i64>) -> cmp::Ordering {
    lhs.iter().cmp(rhs.iter())
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
