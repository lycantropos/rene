use std::hash::Hash;

use crate::geometries::{contracts, Polygon};

use super::types::Multipolygon;

impl<Scalar> PartialEq for Multipolygon<Scalar>
where
    Polygon<Scalar>: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        contracts::are_unique_hashable_sequences_permutationally_equivalent(
            &self.polygons,
            &other.polygons,
        )
    }
}
