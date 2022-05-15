use super::types::Point;

impl<Scalar: PartialEq> PartialEq for Point<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }

    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}
