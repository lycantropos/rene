use std::hash::{Hash, Hasher};

use super::types::Segment;

impl<Scalar: PartialOrd + Hash> Hash for Segment<Scalar> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0.lt(&self.1) {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}
