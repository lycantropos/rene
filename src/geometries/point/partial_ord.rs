use std::cmp::Ordering;

use super::types::Point;

impl<Scalar: PartialOrd> PartialOrd for Point<Scalar> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.lt(other) {
            Ordering::Less
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }

    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0) || self.0.eq(&other.0) && self.1.ge(&other.1)
    }

    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0) || self.0.eq(&other.0) && self.1.gt(&other.1)
    }

    fn le(&self, other: &Self) -> bool {
        self.0.lt(&other.0) || self.0.eq(&other.0) && self.1.le(&other.1)
    }

    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0) || self.0.eq(&other.0) && self.1.lt(&other.1)
    }
}
