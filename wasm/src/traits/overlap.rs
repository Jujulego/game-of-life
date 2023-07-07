pub trait Overlaps<Lhs = Self> {
    fn overlap(&self, lhs: &Lhs) -> bool;
}
