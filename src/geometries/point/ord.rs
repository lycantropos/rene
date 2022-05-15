use std::cmp::Ordering;

use super::types::Point;

impl<Scalar: Ord> Ord for Point<Scalar> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
