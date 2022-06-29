use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::geometries::{utils, Contour};

use super::types::Multisegment;

impl<Scalar: PartialOrd + Hash> Hash for Multisegment<Scalar> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        utils::hash_slice_unordered::<_, H, BuildHasherDefault<DefaultHasher>>(
            &self.segments,
            state,
        );
    }
}
