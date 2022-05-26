use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use rithm::traits::{AdditiveGroup, MultiplicativeMonoid, Signed};

use crate::geometries::operations::hash_slice_unordered;
use crate::geometries::Contour;

use super::types::Polygon;

impl<Scalar: AdditiveGroup + Clone + Hash + MultiplicativeMonoid + Ord + Signed> Hash
    for Polygon<Scalar>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.border.hash(state);
        hash_slice_unordered::<Contour<Scalar>, H, BuildHasherDefault<DefaultHasher>>(
            &self.holes,
            state,
        );
    }
}
