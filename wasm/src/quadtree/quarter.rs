use std::mem;
use na::Point2;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Quarter {
    NorthEast = 0b11,
    NorthWest = 0b01,
    SouthEast = 0b10,
    SouthWest = 0b00,
}

pub fn global_quarter(point: &Point2<i32>) -> Quarter {
    let mut quarter = 0u8;

    if point.x >= 0 {
        quarter |= 0b10;
    }

    if point.y >= 0 {
        quarter |= 0b01;
    }

    unsafe { mem::transmute(quarter) }
}

// Test
#[cfg(test)]
mod test {
    use na::point;
    use super::*;

    #[test]
    fn test_global_quarter() {
        assert_eq!(global_quarter(&point![1, 1]), Quarter::NorthEast);
        assert_eq!(global_quarter(&point![1, -1]), Quarter::SouthEast);
        assert_eq!(global_quarter(&point![-1, 1]), Quarter::NorthWest);
        assert_eq!(global_quarter(&point![-1, -1]), Quarter::SouthWest);
    }
}
