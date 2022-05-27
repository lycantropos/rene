use super::types::Point;

impl<Scalar: PartialEq> PartialEq for Point<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
