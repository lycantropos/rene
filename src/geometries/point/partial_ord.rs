use std::cmp::Ordering;

use super::types::Point;

impl<Scalar: PartialOrd> PartialOrd for Point<Scalar> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.partial_cmp(&other.x)? {
            Ordering::Equal => self.y.partial_cmp(&other.y),
            ordering => Some(ordering),
        }
    }
}
