use std::hash::{Hash, Hasher};

use super::types::Box;

impl<Scalar: Hash> Hash for Box<Scalar> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.max_x.hash(state);
        self.max_y.hash(state);
        self.min_x.hash(state);
        self.min_y.hash(state);
    }
}
