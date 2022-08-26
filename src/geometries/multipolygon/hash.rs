use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasherDefault, Hash, Hasher};

use crate::geometries::{utils, Polygon};

use super::types::Multipolygon;

impl<Scalar> Hash for Multipolygon<Scalar>
where
    Polygon<Scalar>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        utils::hash_slice_unordered::<_, H, BuildHasherDefault<DefaultHasher>>(
            &self.polygons,
            state,
        );
    }
}
