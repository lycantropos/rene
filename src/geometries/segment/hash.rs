use std::hash::{Hash, Hasher};

use super::types::Segment;

impl<Scalar: PartialOrd + Hash> Hash for Segment<Scalar> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.start.lt(&self.end) {
            self.start.hash(state);
            self.end.hash(state);
        } else {
            self.end.hash(state);
            self.start.hash(state);
        }
    }
}
