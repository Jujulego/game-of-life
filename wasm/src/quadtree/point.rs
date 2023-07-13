use na::Point2;
use crate::quadtree::division::Division;

// Utils
impl Division for Point2<i64> {
    #[inline]
    fn anchor(&self) -> &Point2<i64> {
        self
    }

    #[inline]
    fn size(&self) -> u64 {
        1
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    mod point {
        use na::point;
        use super::*;

        #[test]
        fn test_anchor() {
            assert_eq!(point![1, 1].anchor(), &point![1, 1]);
        }

        #[test]
        fn test_size() {
            assert_eq!(point![1, 1].size(), 1);
        }
    }
}
