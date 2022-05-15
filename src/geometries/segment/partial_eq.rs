use super::types::Segment;

impl<Scalar: PartialEq> PartialEq for Segment<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.0 && self.0 == other.1)
    }

    fn ne(&self, other: &Self) -> bool {
        (self.0 != other.0 && self.1 != other.0) || (self.0 != other.1 && self.1 != other.1)
    }
}
