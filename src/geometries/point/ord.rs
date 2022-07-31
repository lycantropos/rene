use std::cmp::Ordering;

use super::types::Point;

impl<Scalar: Ord> Ord for Point<Scalar> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ordering => ordering,
        }
    }
}
