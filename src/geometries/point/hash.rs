use std::hash::{Hash, Hasher};

use super::types::Point;

impl<Scalar: Hash> Hash for Point<Scalar> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}
