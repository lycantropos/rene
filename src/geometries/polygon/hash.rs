use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use crate::geometries::{utils, Contour};

use super::types::Polygon;

impl<Scalar> Hash for Polygon<Scalar>
where
    Contour<Scalar>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.border.hash(state);
        utils::hash_slice_unordered::<
            Contour<Scalar>,
            H,
            BuildHasherDefault<DefaultHasher>,
        >(&self.holes, state);
    }
}
