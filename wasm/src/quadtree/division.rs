use na::Point2;

pub trait Division {
    fn anchor(&self) -> &Point2<i32>;
    fn size(&self) -> u32;
}

impl Division for Point2<i32> {
    #[inline]
    fn anchor(&self) -> &Point2<i32> {
        self
    }

    #[inline]
    fn size(&self) -> u32 {
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
