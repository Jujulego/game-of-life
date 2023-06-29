use na::{point, Point2, vector};
use py::BBox;

/// Build bbox around a single point
pub fn bbox_around(pt: &Point2<i32>) -> BBox<i32, 2> {
    BBox::from_anchor_size(pt, &vector![1, 1])
}

/// Computes bbox of parent node
pub fn parent_bbox(child: &BBox<i32, 2>) -> BBox<i32, 2> {
    let start = child.start_point();
    let end = child.end_point();

    let size = (end - start) * 2;
    let start = point![start.x & !(size.x - 1), start.y & !(size.y - 1)];

    BBox::from_anchor_size(&start, &size)
}

#[cfg(test)]
mod tests {
    use py::traits::BBoxBounded;
    use super::*;

    #[test]
    fn test_bbox_around() {
        assert_eq!(bbox_around(&point![5, 5]), (point![5, 5]..point![6, 6]).bbox());
    }

    mod test_parent_bbox {
        use super::*;

        #[test]
        fn test_on_unit_bbox() {
            assert_eq!(
                parent_bbox(&(point![5, 5]..point![6, 6]).bbox()),
                (point![4, 4]..point![6, 6]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![5, 7]..point![6, 8]).bbox()),
                (point![4, 6]..point![6, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![7, 5]..point![8, 6]).bbox()),
                (point![6, 4]..point![8, 6]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![7, 7]..point![8, 8]).bbox()),
                (point![6, 6]..point![8, 8]).bbox()
            );
        }

        #[test]
        fn test_on_negative_unit_bbox() {
            assert_eq!(
                parent_bbox(&(point![-5, -5]..point![-4, -4]).bbox()),
                (point![-6, -6]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-5, -7]..point![-4, -6]).bbox()),
                (point![-6, -8]..point![-4, -6]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-7, -5]..point![-6, -4]).bbox()),
                (point![-8, -6]..point![-6, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-7, -7]..point![-6, -6]).bbox()),
                (point![-8, -8]..point![-6, -6]).bbox()
            );
        }

        #[test]
        fn test_on_bbox() {
            assert_eq!(
                parent_bbox(&(point![4, 4]..point![6, 6]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![4, 6]..point![6, 8]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![6, 4]..point![8, 6]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![6, 6]..point![8, 8]).bbox()),
                (point![4, 4]..point![8, 8]).bbox()
            );
        }

        #[test]
        fn test_on_negative_bbox() {
            assert_eq!(
                parent_bbox(&(point![-6, -6]..point![-4, -4]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-6, -8]..point![-4, -6]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-8, -6]..point![-6, -4]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
            assert_eq!(
                parent_bbox(&(point![-8, -8]..point![-6, -6]).bbox()),
                (point![-8, -8]..point![-4, -4]).bbox()
            );
        }
    }
}
