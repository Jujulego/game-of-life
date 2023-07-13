use na::Point2;

/// Represents a quadtree division
pub trait Division {
    fn anchor(&self) -> &Point2<i64>;
    fn size(&self) -> u64;
}
