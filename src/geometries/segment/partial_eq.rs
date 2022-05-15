use super::types::Segment;

impl<Scalar: PartialEq> PartialEq for Segment<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.end == other.start && self.start == other.end)
    }

    fn ne(&self, other: &Self) -> bool {
        (self.start != other.start && self.end != other.start)
            || (self.start != other.end && self.end != other.end)
    }
}
