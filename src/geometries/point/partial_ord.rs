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
        self.x.ge(&other.x) || self.x.eq(&other.x) && self.y.ge(&other.y)
    }

    fn gt(&self, other: &Self) -> bool {
        self.x.gt(&other.x) || self.x.eq(&other.x) && self.y.gt(&other.y)
    }

    fn le(&self, other: &Self) -> bool {
        self.x.lt(&other.x) || self.x.eq(&other.x) && self.y.le(&other.y)
    }

    fn lt(&self, other: &Self) -> bool {
        self.x.lt(&other.x) || self.x.eq(&other.x) && self.y.lt(&other.y)
    }
}
