use na::Point2;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Quarter {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

pub fn quarter(origin: &Point2<i32>, point: &Point2<i32>) -> Quarter {
    if point.x >= origin.x && point.y >= origin.y {
        Quarter::NorthEast
    } else if point.x >= origin.x {
        Quarter::SouthEast
    } else if point.y >= origin.y {
        Quarter::NorthWest
    } else {
        Quarter::SouthWest
    }
}

// Test
#[cfg(test)]
mod test {
    use na::point;
    use super::*;

    #[test]
    fn test_quarter() {
        assert_eq!(quarter(&point![0, 0], &point![1, 1]), Quarter::NorthEast);
        assert_eq!(quarter(&point![0, 0], &point![1, -1]), Quarter::SouthEast);
        assert_eq!(quarter(&point![0, 0], &point![-1, 1]), Quarter::NorthWest);
        assert_eq!(quarter(&point![0, 0], &point![-1, -1]), Quarter::SouthWest);
    }
}
