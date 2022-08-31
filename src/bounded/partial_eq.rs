use super::types::Box;

impl<Scalar: PartialEq> PartialEq for Box<Scalar> {
    fn eq(&self, other: &Self) -> bool {
        self.max_x == other.max_x
            && self.max_y == other.max_y
            && self.min_x == other.min_x
            && self.min_y == other.min_y
    }
}
